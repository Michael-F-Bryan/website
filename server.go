// A crappy website made entirely for my personal use.
package website

import (
	"encoding/json"
	"log"
	"net/http"
	"strings"

	"github.com/gorilla/sessions"
	"github.com/tomasen/realip"
	"gopkg.in/mgo.v2/bson"
)

func AuthRequired(store *sessions.CookieStore, users UserData, inner http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		session, _ := store.Get(r, "session")
		rawTok, _ := session.Values["token"].(string)

		if bson.IsObjectIdHex(rawTok) {
			if tok := bson.ObjectIdHex(rawTok); users.TokenIsValid(tok) {
				inner(w, r)
				return
			}
		}

		if strings.Contains("application/json", r.Header.Get("Accept")) {
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(`{"success":false,"error":"Login required"}`))
		} else {
			http.Redirect(w, r, "/forbidden", http.StatusFound)
		}
	}
}

func LoginHandler(users UserData) http.HandlerFunc {
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
				`{"success":false,"error":"please enter the username and password"}`,
				http.StatusUnauthorized)
			return
		}

		token, err := users.LoginUser(request.Username, request.Password)
		if err != nil {
			clientIP := realip.FromRequest(r)
			log.Printf("%s tried to log in with an invalid password (ip: %s)", request.Username, clientIP)
			http.Error(w, `{"success":false,"error":"invalid username or password"}`, http.StatusUnauthorized)
			return
		}

		log.Printf("Logged %s in with the token, %s", request.Username, token)

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
