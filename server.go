package website

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/tomasen/realip"
	"gopkg.in/mgo.v2/bson"
)

type Server struct {
	users UserData
	times Timesheets
}

func NewServer(users UserData, times Timesheets) *Server {
	return &Server{users, times}
}

func (s *Server) LoginHandler(w http.ResponseWriter, r *http.Request) {
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

	token, err := s.users.LoginUser(request.Username, request.Password)
	if err != nil {
		clientIP := realip.FromRequest(r)
		log.Printf("%s tried to log in with an invalid password (ip: %s)", request.Username, clientIP)
		http.Error(w, `{"success":false,"error":"invalid username or password"}`, http.StatusUnauthorized)
		return
	}

	log.Printf("Logged %s in with the token, %s", request.Username, token)

	response := struct {
		success bool          `json:"success"`
		token   bson.ObjectId `json:"token"`
	}{true, token.Id}

	err = json.NewEncoder(w).Encode(response)
	if err != nil {
		log.Printf("Failed to write %v, %s", response, err)
	}
}
