package main

import (
	"fmt"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"path/filepath"
	"time"

	"log"

	"github.com/Michael-F-Bryan/website"
	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
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
			Name:   "dev",
			Usage:  "Proxy all requests to static assets to the provided NPM server",
			EnvVar: "DEV_NPM_SERVER",
		},
	}

	app.Action = start

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

func start(ctx *cli.Context) error {
	args := ParseArgs(ctx)
	log.Printf("Parsed arguments, %#v", args)

	log.Printf("Connecting to mongo at %s", args.DatabaseURL)
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
	store := sessions.NewCookieStore([]byte("super secret key"))

	// It's important that this is before your catch-all route ("/")
	website.RegisterApiRoutes(r, store, conn, conn)
	registerStaticResources(r, args)

	var handler http.Handler = handlers.LoggingHandler(os.Stdout, r)

	if args.DeveloperMode() {
		oldHandler := handler
		handler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set("Cache-Control", "no-cache, no-store")
			oldHandler.ServeHTTP(w, r)
		})
	}

	handler = handlers.CompressHandler(handler)

	return &http.Server{
		Handler:      handler,
		Addr:         fmt.Sprintf("%s:%d", args.Host, args.Port),
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
	}
}

func registerStaticResources(router *mux.Router, args Args) {
	proxyURL, err := url.Parse(args.DevProxy)

	if args.DevProxy != "" && err == nil {
		log.Printf("Proxying static assets to %s", proxyURL)
		router.PathPrefix("/").Handler(httputil.NewSingleHostReverseProxy(proxyURL))
	} else {
		// Serve static assets directly.
		staticServer := http.FileServer(http.Dir(args.Static))
		router.PathPrefix("/static").Handler(staticServer)

		// Catch-all: Serve our JavaScript application's entry-point (index.html).
		router.PathPrefix("/").HandlerFunc(FallbackHandler(args.Entry, args.Static, staticServer))
	}

}

type Args struct {
	Static      string
	Entry       string
	Port        int
	Host        string
	DatabaseURL string
	DevProxy    string
}

func ParseArgs(ctx *cli.Context) Args {
	return Args{
		Static:      ctx.String("static"),
		Entry:       ctx.String("entry"),
		Port:        ctx.Int("port"),
		Host:        ctx.String("host"),
		DatabaseURL: ctx.String("db"),
		DevProxy:    ctx.String("dev"),
	}
}

func (a Args) DeveloperMode() bool {
	return a.DevProxy == ""
}

func FallbackHandler(entrypoint string, staticDir string, staticServer http.Handler) func(w http.ResponseWriter, r *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		filename := filepath.Join(staticDir, r.URL.Path)

		if _, err := os.Stat(filename); err == nil {
			// the file exists, try to serve it from our static assets
			staticServer.ServeHTTP(w, r)
		} else {
			// serve up our endpoint directly from disk
			http.ServeFile(w, r, entrypoint)
		}
	}
}
