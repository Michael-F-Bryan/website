package main

import (
	"database/sql"
	"flag"
	"fmt"
	"log"

	"go.uber.org/zap"
)

type args struct {
	Host     string
	Port     int
	Database string
	Verbose  bool
}

func parseArgs() args {
	var got args

	flag.StringVar(&got.Host, "host", "localhost", "The interface to serve on")
	flag.StringVar(&got.Database, "database", "data.sqlite", "The sqlite database")
	flag.IntVar(&got.Port, "port", 8080, "The port to serve on")
	flag.BoolVar(&got.Verbose, "verbose", false, "Generate verbose output")

	flag.Parse()

	return got
}

func (a args) initializeLogging() *zap.Logger {
	var logger *zap.Logger
	var err error

	if a.Verbose {
		logger, err = zap.NewDevelopment()
	} else {
		logger, err = zap.NewProduction()
	}

	if err != nil {
		log.Fatalf("Unable to initialize the logger: %v", err)
	}

	return logger
}

// openDatabase creates a new connection to the database and applies any
// necessary migrations.
func (a args) openDatabase() (*sql.DB, error) {
	db, err := sql.Open("sqlite3", a.Database)
	if err != nil {
		return nil, err
	}

	if err = db.Ping(); err != nil {
		return nil, err
	}

	return db, nil
}

func (a args) bindAddress() string {
	return fmt.Sprintf("%s:%d", a.Host, a.Port)
}
