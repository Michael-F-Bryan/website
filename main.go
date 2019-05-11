package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"os/signal"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"go.uber.org/zap"
)

func main() {
	args := parseArgs()
	logger := args.initializeLogging()
	defer logger.Sync()

	logger.Info("Starting application", zap.Any("args", args))
	defer logger.Info("Stopping application")

	db, err := args.openDatabase()
	if err != nil {
		logger.Fatal("Unable to open the database", zap.Error(err), zap.String("db", args.Database))
	}
	defer db.Close()

	server := http.Server{
		Addr:         args.bindAddress(),
		Handler:      loadRoutes(db, logger),
		WriteTimeout: time.Second * 15,
		ReadTimeout:  time.Second * 15,
		IdleTimeout:  time.Second * 60,
	}
	go runOnCtrlC(server.Shutdown)

	logger.Info("Serving", zap.String("bind", server.Addr))
	if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		logger.Fatal("Serving failed", zap.Error(err))
	}
}

func runOnCtrlC(callback func(context.Context) error) {
	ch := make(chan os.Signal)
	signal.Notify(ch, os.Interrupt)

	<-ch

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := callback(ctx); err != nil {
		log.Fatalf("Unable to stop the server: %v", err)
	}
}
