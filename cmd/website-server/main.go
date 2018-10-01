package main

import (
	"fmt"
	"net/http"
	"net/http/httputil"
	"net/url"
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
		cli.StringFlag{
			Name:  "host",
			Value: "localhost",
			Usage: "The interface to serve on",
		},
		cli.IntFlag{
			Name:  "port",
			Value: 8000,
			Usage: "The port to listen on",
		},
		cli.StringFlag{
			Name:  "dev",
			Usage: "Proxy all requests to static assets to the provided NPM server",
		},
	}

	app.Action = start

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

func start(ctx *cli.Context) error {
	args := ParseArgs(ctx)

	conn, err := website.NewDatabase(args.DatabaseURL)
	if err != nil {
		log.Fatalf("Unable to create the server, %v", err)
	}

	srv := CreateServer(conn, args)

	log.Printf("Serving on %s", srv.Addr)
	return srv.ListenAndServe()
}

func CreateServer(conn *website.Database, args Args) *http.Server {
	r := mux.NewRouter()

	// It's important that this is before your catch-all route ("/")
	api := r.PathPrefix("/api/").Subrouter()
	api.HandleFunc("/login", website.LoginHandler(conn)).Methods("POST")

	if proxyURL, err := url.Parse(args.StaticProxy); err == nil {
		log.Printf("Proxying static assets to %s", proxyURL)
		r.PathPrefix("/").Handler(httputil.NewSingleHostReverseProxy(proxyURL))
	} else {
		// Serve static assets directly.
		r.PathPrefix("/static").Handler(http.FileServer(http.Dir(args.Static)))

		// Catch-all: Serve our JavaScript application's entry-point (index.html).
		r.PathPrefix("/").HandlerFunc(IndexHandler(args.Entry))
	}

	return &http.Server{
		Handler:      handlers.LoggingHandler(os.Stdout, r),
		Addr:         fmt.Sprintf("%s:%d", args.Host, args.Port),
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
	}
}

type Args struct {
	Static      string
	Entry       string
	Port        int
	Host        string
	DatabaseURL string
	StaticProxy string
}

func ParseArgs(ctx *cli.Context) Args {
	return Args{
		Static:      ctx.String("static"),
		Entry:       ctx.String("entry"),
		Port:        ctx.Int("port"),
		Host:        ctx.String("host"),
		DatabaseURL: ctx.String("db"),
		StaticProxy: ctx.String("dev"),
	}
}

func IndexHandler(entrypoint string) func(w http.ResponseWriter, r *http.Request) {
	fn := func(w http.ResponseWriter, r *http.Request) {
		log.Printf("Serving %s (%s)", r.URL, entrypoint)
		http.ServeFile(w, r, entrypoint)
	}

	return http.HandlerFunc(fn)
}
