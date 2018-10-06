package website

import (
	"encoding/json"
	"errors"
	"io/ioutil"
	"net/http"
	"net/http/cookiejar"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
	"gopkg.in/mgo.v2/bson"
)

func TestLogIntoTheServer(t *testing.T) {
	store := sessions.NewCookieStore([]byte("super secret key"))
	users := newMockData()
	handler := LoginHandler(store, users)
	jason := `{"username":"admin","password":"password1"}`

	req, err := http.NewRequest("POST", "/api/login", strings.NewReader(jason))
	if err != nil {
		t.Fatal(err)
	}

	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	resp := w.Result()

	if status := w.Code; status != http.StatusOK {
		t.Errorf("Expected 200 OK, got %s", http.StatusText(resp.StatusCode))
	}

	got := struct {
		Success bool          `json:"success"`
		Token   bson.ObjectId `json:"token"`
	}{}
	err = json.NewDecoder(resp.Body).Decode(&got)
	if err != nil {
		t.Error(err)
	}

	if !got.Success {
		t.Error("Not successful")
	}
}

func TestAuthRequired(t *testing.T) {
	store := sessions.NewCookieStore([]byte("super secret key"))
	users := newMockData()
	handler := AuthRequired(store, users, func(w http.ResponseWriter, r *http.Request) {
		panic("should be blocked")
	})

	req, err := http.NewRequest("GET", "/", nil)
	req.Header.Set("Accept", "application/json")
	if err != nil {
		t.Fatal(err)
	}

	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	resp := w.Result()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		t.Error(err)
	}

	if status := w.Code; status != http.StatusForbidden {
		t.Errorf("Expected 401 Forbidden, got %s", http.StatusText(resp.StatusCode))
	}

	if !strings.Contains(string(body), "error") {
		t.Errorf("The response should mention an error, got \"%v\"", string(body))
	}
}

func TestLogout(t *testing.T) {
	server, client, _, users, _ := integrationTestSetup()
	defer server.Close()

	body := `{"username":"admin","password":"password1"}`
	res, err := client.Post(server.URL+"/api/login", "application/json", strings.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}
	defer res.Body.Close()

	if len(users.tokens) != 1 {
		t.Errorf("There should only be 1 token, found %d", len(users.tokens))
	}
	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected 200 OK but got %s", http.StatusText(res.StatusCode))
	}

	body = ``
	res, err = client.Post(server.URL+"/api/logout", "application/json", strings.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}
	defer res.Body.Close()

	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected 200 OK but got %d %s", res.StatusCode, http.StatusText(res.StatusCode))
	}

	for _, token := range users.tokens {
		if !token.Deleted {
			t.Errorf("The token should have been deleted, found %v", token)
		}
	}
}

func TestPingUpdatesLastSeen(t *testing.T) {
	server, client, _, users, _ := integrationTestSetup()
	defer server.Close()

	// we need to be logged in
	body := `{"username":"admin","password":"password1"}`
	res, err := client.Post(server.URL+"/api/login", "application/json", strings.NewReader(body))
	if err != nil || res.StatusCode != http.StatusOK {
		t.Fatal("Couldn't login")
	}

	res, err = client.Get(server.URL + "/api/ping")
	if err != nil {
		t.Fatal(err)
	}
	defer res.Body.Close()

	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected 200 OK but got %d %s", res.StatusCode, http.StatusText(res.StatusCode))
	}

	ping := Ping{}
	if err := json.NewDecoder(res.Body).Decode(&ping); err != nil {
		t.Error(err)
	}

	token := users.tokens[ping.Token]
	if !token.LastSeen.After(ping.PreviousLastSeen) {
		t.Errorf("The LastSeen field should have been updated. %s is not after %s",
			token.LastSeen, ping.PreviousLastSeen)
	}

	if ping.Username != "admin" {
		t.Errorf(`Username should be admin, found "%s"`, ping.Username)
	}
}

func integrationTestSetup() (*httptest.Server, *http.Client, *sessions.CookieStore, *MockData, *MockTimes) {
	store := sessions.NewCookieStore([]byte("super secret key"))
	users := newMockData()
	times := newMockTimes()
	router := mux.NewRouter()
	RegisterApiRoutes(router, store, users, times)
	server := httptest.NewServer(router)

	client := server.Client()
	jar, _ := cookiejar.New(nil)
	client.Jar = jar
	client.CheckRedirect = func(req *http.Request, via []*http.Request) error {
		return http.ErrUseLastResponse
	}

	return server, client, store, users, times
}

type MockData struct {
	users  map[string]User
	tokens map[bson.ObjectId]Token
}

func newMockData() *MockData {
	d := &MockData{
		users:  make(map[string]User),
		tokens: make(map[bson.ObjectId]Token),
	}

	_, _ = d.CreateUser("admin", "password1")

	return d
}

func (m *MockData) CreateUser(username, password string) (User, error) {
	user, err := NewUser(username, password)

	if err == nil {
		m.users[username] = user
	}

	return user, err
}

func (m *MockData) LoginUser(username, password string) (Token, error) {
	user, ok := m.users[username]
	if !ok {
		return Token{}, errors.New("Nonexistent user")
	}

	if !user.PasswordIsValid(password) {
		return Token{}, errors.New("Invalid password")
	}

	now := time.Now()
	tok := Token{
		Id:       bson.NewObjectId(),
		User:     user.Id,
		Created:  now,
		LastSeen: now,
	}
	m.tokens[tok.Id] = tok

	return tok, nil
}

func (m *MockData) GetUsers() ([]string, error) {
	panic("Not Implemented")
}

func (m *MockData) GetUserById(id bson.ObjectId) (*User, error) {
	for _, user := range m.users {
		if user.Id == id {
			return &user, nil
		}
	}

	return nil, errors.New("Not Found")
}

func (m *MockData) DeleteUser(username string) error {
	panic("Not Implemented")
}

func (m *MockData) Logout(id bson.ObjectId) error {
	token, exists := m.tokens[id]
	if !exists {
		return errors.New("Invalid token")
	}

	token.Deleted = true
	m.tokens[id] = token
	return nil
}

func (m *MockData) GetToken(id bson.ObjectId) *Token {
	token := m.tokens[id]
	return &token
}

func (m *MockData) UpdateLastSeen(id bson.ObjectId, now time.Time) error {
	if token, exists := m.tokens[id]; exists {
		token.LastSeen = now
		m.tokens[id] = token
		return nil
	}

	return errors.New("Token is not valid")
}

type MockTimes struct{}

func newMockTimes() *MockTimes {
	return &MockTimes{}
}

func (m *MockTimes) GetEntryById(id bson.ObjectId) (Entry, error) {
	panic("Not Implemented")
}

func (m *MockTimes) UpdateOrInsertTimesheet(entry Entry) error {
	panic("Not Implemented")
}

func (m *MockTimes) DeleteTimesheet(entry Entry) error {
	panic("Not Implemented")
}

func (m *MockTimes) NumTimesheets() (int, error) {
	panic("Not Implemented")
}
