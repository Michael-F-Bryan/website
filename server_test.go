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
	server, client, _, _, _ := integrationTestSetup()
	defer server.Close()

	body := `{"username":"admin","password":"password1"}`
	res, err := client.Post(server.URL+"/api/login", "application/json", strings.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}

	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected 200 OK but got %s", http.StatusText(res.StatusCode))
	}

	body = ``
	res, err = client.Post(server.URL+"/api/logout", "application/json", strings.NewReader(body))
	if err != nil {
		t.Fatal(err)
	}

	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected 200 OK but got %d %s", res.StatusCode, http.StatusText(res.StatusCode))
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

func newMockData() *MockData {
	d := &MockData{
		users: make(map[string]User),
	}

	_, _ = d.CreateUser("admin", "password1")

	return d
}

type MockData struct {
	users  map[string]User
	tokens []Token
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

	tok := Token{Id: bson.NewObjectId(), User: user.Id}
	m.tokens = append(m.tokens, tok)

	return tok, nil
}

func (m *MockData) GetUsers() ([]string, error) {
	panic("Not Implemented")
}

func (m *MockData) DeleteUser(username string) error {
	panic("Not Implemented")
}

func (m *MockData) Logout(tok bson.ObjectId) error {
	panic("Not Implemented")
}

func (m *MockData) GetToken(tok bson.ObjectId) *Token {
	for _, token := range m.tokens {
		if token.Id == tok {
			return &token
		}
	}

	return nil
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
