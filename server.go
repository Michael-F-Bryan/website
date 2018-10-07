// Package website is just a crappy website made entirely for my personal use.
package website

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"time"

	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
	"github.com/tomasen/realip"
	"gopkg.in/mgo.v2/bson"
)

func RegisterApiRoutes(router *mux.Router, store *sessions.CookieStore, users UserData, times Timesheets) {
	api := router.PathPrefix("/api").Subrouter()
	api.HandleFunc("/login", LoginHandler(store, users)).Methods("POST").Headers("Content-Type", "application/json")

	logout := AuthRequired(store, users, LogoutHandler(store, users))
	api.HandleFunc("/logout", logout).Methods("POST").Headers("Content-Type", "application/json")

	api.HandleFunc("/ping", PingHandler(store, users)).Methods("GET")

	newEntry := AuthRequired(store, users, NewEntryHandler(times))
	api.HandleFunc("/timesheets/new", newEntry).Methods("POST").Headers("Content-Type", "application/json")

	api.NotFoundHandler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Error(w, `{"success":false,"error":"No such endpoint"}`, http.StatusNotFound)
	})
}

// AuthRequired wraps an existing handler and will validate that the user is
// authenticated.
//
// If the user is authenticated, the token ID and user ID will be added to the
// request's context (as "token" and "user" respectively).
//
// Otherwise people are served a "Forbidden" page.
func AuthRequired(store *sessions.CookieStore, users UserData, inner http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		session, _ := store.Get(r, "session")
		rawTok, _ := session.Values["token"].(string)

		if bson.IsObjectIdHex(rawTok) {
			tokenId := bson.ObjectIdHex(rawTok)

			if tok := users.GetToken(tokenId); tok != nil {
				ctx := r.Context()
				ctx = context.WithValue(ctx, "token", tok.Id)
				ctx = context.WithValue(ctx, "user", tok.User)
				r = r.WithContext(ctx)
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

		if err := users.UpdateLastSeen(token.Id, time.Now()); err != nil {
			log.Printf("Unable to update last seen for %s, %s (token: %s)", request.Username, err, token.Id)
			http.Error(w, `{"success":false,"error":"Internal server error"}`, http.StatusInternalServerError)
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

func LogoutHandler(store *sessions.CookieStore, users UserData) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		session, _ := store.Get(r, "session")
		rawTok, _ := session.Values["token"].(string)

		if bson.IsObjectIdHex(rawTok) {
			if tok := bson.ObjectIdHex(rawTok); users.GetToken(tok) != nil {
				if err := users.Logout(tok); err != nil {
					log.Printf("Unable to log out %s, %s", tok, err)
					http.Error(w, `{"success":false,"error":"Unable to log out"}`, http.StatusInternalServerError)
					return
				}
			}
		}

		session.Values["token"] = ""

		if err := session.Save(r, w); err != nil {
			log.Printf("Unable to save the session, %s", err)
			w.Write([]byte(`{"success":false,"error":"Couldn't remove the cookie"}`))
			return
		}

		w.Write([]byte(`{"success":true}`))
	}
}

type Ping struct {
	LoggedIn         bool          `json:"logged-in"`
	Token            bson.ObjectId `json:"token"`
	Username         string        `json:"username"`
	PreviousLastSeen time.Time     `json:"previously-seen"`
}

func (p Ping) String() string {
	return fmt.Sprintf(`Ping { LoggedIn: %v, Token: %s, Username: "%s", PreviousLastSeen: %s }`,
		p.LoggedIn,
		p.Token.Hex(),
		p.Username,
		p.PreviousLastSeen)
}

func PingHandler(store *sessions.CookieStore, users UserData) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ping := Ping{}
		session, _ := store.Get(r, "session")
		rawTok, _ := session.Values["token"].(string)

		if bson.IsObjectIdHex(rawTok) {
			id := bson.ObjectIdHex(rawTok)
			if tok := users.GetToken(id); tok != nil {
				ping.LoggedIn = true
				ping.PreviousLastSeen = tok.LastSeen
				ping.Token = id

				if err := users.UpdateLastSeen(id, time.Now()); err != nil {
					log.Printf("Unable to update last seen for %s, %s", tok.Id, err)
					http.Error(w, `{"success":false,"error":"Internal server error"}`, http.StatusInternalServerError)
					return
				}

				user, err := users.GetUserById(tok.User)
				if err != nil {
					log.Printf("Unable to fetch the user corresponding to %#v, %s", tok, err)
					http.Error(w, `{"success":false,"error":"Internal server error"}`, http.StatusInternalServerError)
					return
				} else {
					ping.Username = user.Name
				}
			}
		}

		log.Printf("Sending back a %s", ping)

		if err := json.NewEncoder(w).Encode(&ping); err != nil {
			log.Printf("Unable to encode the ping response, %s", err)
		}
	}
}

func NewEntryHandler(times Timesheets) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		userId, ok := r.Context().Value("user").(bson.ObjectId)
		if !ok {
			log.Print("The user ID wasn't provided as part of the request context")
			http.Error(w, `{"success":false,"error":"internal server error"}`, http.StatusInternalServerError)
			return
		}

		var entry Entry
		if err := json.NewDecoder(r.Body).Decode(&entry); err != nil {
			log.Printf("Unable to parse the request, %s", err)
			http.Error(w, fmt.Sprintf(`{"success":false,"error":"unable to parse the request"}`, err), http.StatusBadRequest)
			return
		}

		if entry.ID == "" {
			entry.ID = bson.NewObjectId()
		}
		entry.User = userId

		if err := times.UpdateOrInsertTimesheet(entry); err != nil {
			log.Printf("Error updating/inserting entry %v, %s", entry, err)
			http.Error(w, `{"success":false,"error":"internal server error"}`, http.StatusInternalServerError)
			return
		}

		response := struct {
			Success bool  `json:"success"`
			Entry   Entry `json:"entry"`
		}{Entry: entry, Success: true}
		if err := json.NewEncoder(w).Encode(&response); err != nil {
			log.Printf("Unable to serialize the response, %s", err)
		}
	}
}
