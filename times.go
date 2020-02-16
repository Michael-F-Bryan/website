package main

import (
	"database/sql"
	"net/http"

	"go.uber.org/zap"
)

func listTimes(db *sql.DB, logger *zap.Logger) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		p, err := newJwtPrincipal(r, globalKeyFunc)
		if err != nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			logger.Warn("Unable to load the JWT", zap.Error(err))
			return
		}
		if !p.HasPermission(PermissionTimesRead) {
			
		}
	})
}
