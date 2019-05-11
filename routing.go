package main

import (
	"database/sql"
	"net/http"
	"time"

	rice "github.com/GeertJohan/go.rice"
	"github.com/gorilla/mux"
	"go.uber.org/zap"
)

// Principal provides a method for authentication and authorization
type Principal interface {
	Authenticated() bool
	UserID() (id int, ok bool)
	HasPermission(permission string) bool
}

func loadRoutes(db *sql.DB, logger *zap.Logger) http.Handler {
	r := mux.NewRouter()
	r.Use(loggingMiddleware(logger))

	// The UI relies on the fact that any unknown URL shows the index page
	box := rice.MustFindBox("frontend/dist")
	fs := box.HTTPBox()
	r.PathPrefix("/").Handler(http.FileServer(fs))
	r.NotFoundHandler = logged(indexFallback(fs), logger)

	return r
}

func logged(handler http.Handler, logger *zap.Logger) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		spy := spyResponseWriter{inner: w, code: http.StatusOK}
		start := time.Now()

		handler.ServeHTTP(&spy, r)

		logger.Info("Served a request",
			zap.String("remote-addr", r.RemoteAddr),
			zap.Stringer("url", r.URL),
			zap.String("method", r.Method),
			zap.Duration("response-time", time.Since(start)),
			zap.Int("response-code", spy.code),
			zap.Int("response-length", spy.bytesWritten),
			zap.String("user-agent", r.UserAgent()),
			zap.String("referrer", r.Referer()),
			zap.String("host", r.Host))

		logger.Debug("Extra info", zap.Any("headers", r.Header))
	})
}

func indexFallback(fs *rice.HTTPBox) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Write(fs.MustBytes("index.html"))
	})
}

func loggingMiddleware(logger *zap.Logger) mux.MiddlewareFunc {
	return func(next http.Handler) http.Handler {
		return logged(next, logger)
	}
}

type spyResponseWriter struct {
	inner        http.ResponseWriter
	code         int
	bytesWritten int
}

func (s *spyResponseWriter) Header() http.Header {
	return s.inner.Header()
}

func (s *spyResponseWriter) WriteHeader(code int) {
	s.code = code
	s.inner.WriteHeader(code)
}

func (s *spyResponseWriter) Write(data []byte) (int, error) {
	bytesWritten, err := s.inner.Write(data)
	s.bytesWritten += bytesWritten

	return bytesWritten, err
}
