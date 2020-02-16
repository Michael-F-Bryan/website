package main

import (
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/dgrijalva/jwt-go"
	"github.com/stretchr/testify/assert"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
)

func TestNoTokenPresent(t *testing.T) {
	tf := setup(t)
	route := authRequired(http.HandlerFunc(ok), tf.db, tf.KeyFunc, tf.logger)
	w := httptest.NewRecorder()
	r, _ := http.NewRequest(http.MethodGet, "/", nil)
	r.Header.Del("Authorization")

	route.ServeHTTP(w, r)

	resp := w.Result()
	assert.Equal(t, http.StatusUnauthorized, resp.StatusCode)
}

func TestValidToken(t *testing.T) {
	tf := setup(t)
	route := authRequired(http.HandlerFunc(ok), tf.db, tf.KeyFunc, tf.logger)
	w := httptest.NewRecorder()
	r, _ := http.NewRequest(http.MethodGet, "/", nil)
	claims := Claims{}
	claims.Subject = "42"
	claims.ExpiresAt = time.Now().Add(1 * time.Minute).Unix()
	header, err := jwt.NewWithClaims(jwt.SigningMethodHS256, claims).SignedString(tf.key)
	assert.NoError(t, err)
	r.Header.Set("Authorization", "Bearer "+header)

	route.ServeHTTP(w, r)

	resp := w.Result()
	assert.Equal(t, http.StatusOK, resp.StatusCode)
}

func ok(w http.ResponseWriter, r *http.Request) { w.WriteHeader(http.StatusOK) }

type testFixtures struct {
	logger *zap.Logger
	db     *sql.DB
	key    []byte
}

func (t testFixtures) KeyFunc(*jwt.Token) (interface{}, error) {
	return t.key, nil
}

func setup(t *testing.T) testFixtures {
	logger := zaptest.NewLogger(t)

	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatal(err)
	}

	assert.NoError(t, applyMigrations(db))

	return testFixtures{logger, db, []byte("Super secret key")}
}
