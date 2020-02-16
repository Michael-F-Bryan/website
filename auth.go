package main

import (
	"crypto/rand"
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"strconv"
	"strings"

	"github.com/dgrijalva/jwt-go"
	"github.com/pkg/errors"
	"go.uber.org/zap"
)

const (
	PermissionTimesRead = "TIMES/READ"
)

var randomKey []byte

func init() {
	randomKey = make([]byte, 32)
	if _, err := rand.Read(randomKey); err != nil {
		log.Fatalf("Unable to generate a random key: %v", err)
	}
}

func globalKeyFunc(*jwt.Token) (interface{}, error) {
	return randomKey, nil
}

// Principal provides a method for authentication and authorization
type Principal interface {
	Authenticated() bool
	UserID() (id int, ok bool)
	HasPermission(permission string) bool
}

// Claims wraps a set of jwt.StandardClaims, adding on our own custom claims.
type Claims struct {
	Permissions []string `json:"permissions"`
	jwt.StandardClaims
}

func (c Claims) Valid() error {
	err := c.StandardClaims.Valid()
	if err != nil {
		return err
	}

	_, err = strconv.ParseInt(c.Subject, 10, 64)
	if err != nil {
		return errors.Wrap(err, "Invalid user ID")
	}

	return nil
}

type jwtPrincipal struct {
	claims Claims
}

func newJwtPrincipal(r *http.Request, keyfunc jwt.Keyfunc) (Principal, error) {
	header := r.Header.Get("Authorization")

	if header == "" {
		return nil, errors.New("No Authorization header found")
	}

	pieces := strings.SplitN(header, " ", 2)

	if len(pieces) != 2 || strings.ToLower(pieces[0]) != "bearer" {
		return nil, errors.New("Expected bearer authorization")
	}

	claims := Claims{}
	_, err := jwt.ParseWithClaims(pieces[1], &claims, keyfunc)
	if err != nil {
		return nil, err
	}

	if err = claims.Valid(); err != nil {
		return nil, err
	}

	return jwtPrincipal{claims: claims}, nil
}

func (j jwtPrincipal) Authenticated() bool {
	_, ok := j.UserID()
	return ok
}

func (j jwtPrincipal) UserID() (id int, ok bool) {
	id, err := strconv.Atoi(j.claims.Subject)
	return id, err == nil
}

func (j jwtPrincipal) HasPermission(permission string) bool {
	for _, perm := range j.claims.Permissions {
		if permission == perm {
			return true
		}
	}

	return false
}

func expectPermissions(handler http.Handler, logger *zap.Logger, permissions ...string) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		p, err := newJwtPrincipal(r, globalKeyFunc)
		if err != nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			logger.Warn("Unable to load the JWT", zap.Error(err))
			return
		}

		for _, perm := range permissions {
			if !p.HasPermission(perm) {
				id, _ := p.UserID()
				logger.Warn("User is missing permissions",
					zap.Int("user-id", id),
					zap.String("missing-permission", perm))
				http.Error(w, fmt.Sprintf("Missing the %q permission", perm), http.StatusForbidden)
				return
			}
		}

		handler.ServeHTTP(w, r)
	})
}

func login(db *sql.DB, logger *zap.Logger) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if err := r.ParseForm(); err != nil {
			logger.Warn("Unable to parse form data", zap.Error(err))
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		username := r.Form.Get("username")
		password := r.Form.Get("password")
		if username == "" || password == "" {
			http.Error(w, "Invalid form", http.StatusBadRequest)
			return
		}
	})
}

func getUserById()