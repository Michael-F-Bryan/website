package main

import (
	"encoding/json"
	"flag"
	"net/http"
	"os"
	"time"

	"log"

	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"github.com/tomasen/realip"
)

func main() {
	var entry string
	var static string
	var port string

	flag.StringVar(&entry, "entry", "./frontend/build/index.html", "the entrypoint to serve.")
	flag.StringVar(&static, "static", "./frontend/build", "the directory to serve static files from.")
	flag.StringVar(&port, "port", "8000", "the `port` to listen on.")
	flag.Parse()

	s := NewServer()

	r := mux.NewRouter()

	// It's important that this is before your catch-all route ("/")
	api := r.PathPrefix("/api/").Subrouter()
	api.HandleFunc("/login", s.LoginHandler).Methods("POST")

	// Serve static assets directly.
	r.PathPrefix("/static").Handler(http.FileServer(http.Dir(static)))

	// Catch-all: Serve our JavaScript application's entry-point (index.html).
	r.PathPrefix("/").HandlerFunc(IndexHandler(entry))

	srv := &http.Server{
		Handler:      handlers.LoggingHandler(os.Stdout, r),
		Addr:         "127.0.0.1:" + port,
		WriteTimeout: 15 * time.Second,
		ReadTimeout:  15 * time.Second,
	}

	log.Printf("Serving on %s", srv.Addr)

	log.Fatal(srv.ListenAndServe())
}

type Server struct {
	auth *Auth
}

func NewServer() *Server {
	return &Server{
		auth: NewAuth(),
	}
}

func IndexHandler(entrypoint string) func(w http.ResponseWriter, r *http.Request) {
	fn := func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, entrypoint)
	}

	return http.HandlerFunc(fn)
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

	token, err := s.auth.LoginUser(request.Username, request.Password)
	if err != nil {
		clientIP := realip.FromRequest(r)
		log.Printf("%s tried to log in with an invalid password (ip: %s)", request.Username, clientIP)
		http.Error(w, `{"success":false,"error":"invalid username or password"}`, http.StatusUnauthorized)
		return
	}

	log.Printf("Logged %s in with the token, %d", request.Username, token)

	response := struct {
		success bool  `json:"success"`
		token   Token `json:"token"`
	}{true, token}

	err = json.NewEncoder(w).Encode(response)
	if err != nil {
		log.Printf("Failed to write %v, %s", response, err)
	}
}
