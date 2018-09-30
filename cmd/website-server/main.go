package main

import (
	"fmt"
	"net/http"
	"os"
	"time"

	"log"

	"github.com/Michael-F-Bryan/website"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"github.com/urfave/cli"
)

func main() {
	app := cli.NewApp()
	app.Usage = "The server for my crappy website"
	app.EnableBashCompletion = true
	app.Version = website.VERSION
	app.Authors = []cli.Author{
		{Name: "Michael-F-Bryan", Email: "michaelfbryan@gmail.com"},
	}

	app.Flags = []cli.Flag{
		cli.StringFlag{
			Name:   "db",
			Usage:  "The database URL to use when connecting to MongoDB",
			Value:  "localhost:27017",
			EnvVar: "DATABASE_URL",
		},
		cli.StringFlag{
			Name:  "entry, e",
			Value: "./frontend/build/index.html",
			Usage: "The entrypoint to serve",
		},
		cli.StringFlag{
			Name:  "static",
			Value: "./frontend/build",
			Usage: "The directory to serve static files from",
		},
		cli.IntFlag{
			Name:  "port",
			Value: 8000,
			Usage: "The port to listen on",
		},
	}

	app.Action = start

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

func start(ctx *cli.Context) error {
	dbUrl := ctx.String("db")
	static := ctx.String("static")
	entry := ctx.String("entry")
	port := ctx.Int("port")

	conn, err := website.NewDatabase(dbUrl)
	if err != nil {
		log.Fatalf("Unable to create the server, %v", err)
	}

	srv := CreateServer(conn, static, entry, port)

	log.Printf("Serving on %s", srv.Addr)
	return srv.ListenAndServe()
}

func CreateServer(conn *website.Database, static, entry string, port int) *http.Server {
	r := mux.NewRouter()

	// It's important that this is before your catch-all route ("/")
	api := r.PathPrefix("/api/").Subrouter()
	api.HandleFunc("/login", website.LoginHandler(conn)).Methods("POST")

	// Serve static assets directly.
	r.PathPrefix("/static").Handler(http.FileServer(http.Dir(static)))

	// Catch-all: Serve our JavaScript application's entry-point (index.html).
	r.PathPrefix("/").HandlerFunc(IndexHandler(entry))

	return &http.Server{
		Handler:      handlers.LoggingHandler(os.Stdout, r),
		Addr:         fmt.Sprintf("127.0.0.1:%d", port),
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
	}
}

func IndexHandler(entrypoint string) func(w http.ResponseWriter, r *http.Request) {
	fn := func(w http.ResponseWriter, r *http.Request) {
		log.Printf("Serving %s (%s)", r.URL, entrypoint)
		http.ServeFile(w, r, entrypoint)
	}

	return http.HandlerFunc(fn)
}
