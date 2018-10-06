// Package website is just a crappy website made entirely for my personal use.
package website

import (
	"encoding/json"
	"log"
	"net/http"
	"strings"

	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
	"github.com/tomasen/realip"
	"gopkg.in/mgo.v2/bson"
)

func RegisterApiRoutes(router *mux.Router, store *sessions.CookieStore, users UserData, times Timesheets) {
	api := router.PathPrefix("/api").Subrouter()
	api.HandleFunc("/login", LoginHandler(store, users)).Methods("POST").Headers("Content-Type", "application/json")

	logout := AuthRequired(store, users, LogoutHandler(store))
	api.HandleFunc("/logout", logout).Methods("POST").Headers("Content-Type", "application/json")

	api.HandleFunc("/ping", Ping(store, users)).Methods("GET")
}

func AuthRequired(store *sessions.CookieStore, users UserData, inner http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		session, _ := store.Get(r, "session")
		rawTok, _ := session.Values["token"].(string)

		if bson.IsObjectIdHex(rawTok) {
			if tok := bson.ObjectIdHex(rawTok); users.GetToken(tok) != nil {
				inner(w, r)
				return
			}
		}

		log.Printf("%s tried to access %s without authentication", realip.FromRequest(r), r.URL)

		if acceptsJson(r.Header) {
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(`{"success":false,"error":"Login required"}`))
		} else {
			http.Redirect(w, r, "/forbidden", http.StatusFound)
		}
	}
}

func acceptsJson(header http.Header) bool {
	var fields []string
	if accept := header.Get("Accept"); accept != "" {
		fields = append(fields, strings.ToLower(accept))
	}
	if contentType := header.Get("Contet-Type"); contentType != "" {
		fields = append(fields, strings.ToLower(contentType))
	}

	for _, field := range fields {
		if strings.Contains(field, "application/json") {
			return true
		}
	}

	return false
}

func LoginHandler(store *sessions.CookieStore, users UserData) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		request := struct {
			Username string `json:"username"`
			Password string `json:"password"`
		}{}

		if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
			http.Error(w, `{"success":false,"error":"invalid JSON"}`, http.StatusBadRequest)
			return
		}

		if request.Username == "" || request.Password == "" {
			http.Error(w,
				`{"success":false,"error":"please enter a username and password"}`,
				http.StatusUnauthorized)
			return
		}

		token, err := users.LoginUser(request.Username, request.Password)
		if err != nil {
			clientIP := realip.FromRequest(r)
			log.Printf("%s failed to log in, %s (ip: %s)", request.Username, err, clientIP)
			http.Error(w, `{"success":false,"error":"invalid username or password"}`, http.StatusUnauthorized)
			return
		}

		log.Printf("Logged %s in with the token, %s", request.Username, token)

		session, _ := store.Get(r, "session")
		session.Values["token"] = token.Id.Hex()
		if err = session.Save(r, w); err != nil {
			log.Printf("Unable to save the session, %s", err)
		}

		response := struct {
			Success bool          `json:"success"`
			Token   bson.ObjectId `json:"token"`
		}{true, token.Id}

		err = json.NewEncoder(w).Encode(response)
		if err != nil {
			log.Printf("Failed to write %v, %s", response, err)
		}
	}
}

func LogoutHandler(store *sessions.CookieStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		session, _ := store.Get(r, "session")
		session.Values["token"] = ""
		if err := session.Save(r, w); err != nil {
			log.Printf("Unable to save the session, %s", err)
			w.Write([]byte(`{"success":false,"error":"Couldn't remove the cookie"}`))
		} else {
			w.Write([]byte(`{"success":true}`))
		}
	}
}

func Ping(store *sessions.CookieStore, users UserData) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ping := struct {
			LoggedIn bool   `json:"logged-in"`
			Username string `json:"username"`
		}{}

		session, _ := store.Get(r, "session")
		rawTok, _ := session.Values["token"].(string)

		if bson.IsObjectIdHex(rawTok) {
			id := bson.ObjectIdHex(rawTok)
			if tok := users.GetToken(id); tok != nil {
				ping.LoggedIn = true
			}
		}
	}
}
