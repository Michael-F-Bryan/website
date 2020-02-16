package main

import (
	"database/sql"
	"net/http"
	"strings"
	"time"

	rice "github.com/GeertJohan/go.rice"
	"github.com/dgrijalva/jwt-go"
	"github.com/gorilla/mux"
	"go.uber.org/zap"
)

func loadRoutes(db *sql.DB, logger *zap.Logger) http.Handler {
	r := mux.NewRouter()
	r.Use(loggingMiddleware(logger))

	api := r.PathPrefix("/api/").Subrouter()
	api.Handle("/login/", login(db, logger)).Methods(http.MethodPost)
	api.Handle("/times/", expectPermissions(listTimes(db, logger), logger, PermissionTimesRead)).
		Methods(http.MethodGet)

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

// authRequired wrapps a http.Handler and ensures the JWT in its Authorization
// header is present and valid.
func authRequired(handler http.Handler, db *sql.DB, keyFunc jwt.Keyfunc, logger *zap.Logger) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		header := r.Header.Get("Authorization")
		if header == "" {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		pieces := strings.SplitN(header, " ", 2)

		if len(pieces) != 2 || strings.ToLower(pieces[0]) != "bearer" {
			http.Error(w, "Please provide a JWT using Bearer authorization", http.StatusUnauthorized)
			return
		}

		claims := Claims{}
		token, err := jwt.ParseWithClaims(pieces[1], &claims, keyFunc)
		if err != nil {
			logger.Warn("Looks like someone's playing silly buggers with JWTs...",
				zap.String("token", pieces[1]))
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		expires := time.Unix(claims.ExpiresAt, 0)
		logger.Debug("Parsed a JSON Web Token",
			zap.Stringer("url", r.URL),
			zap.Any("token", token),
			zap.String("remote-addr", r.RemoteAddr),
			zap.Time("expires", expires))

		if time.Now().After(expires) {
			http.Error(w, "Expired token", http.StatusUnauthorized)
			return
		}

		handler.ServeHTTP(w, r)
	})
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
