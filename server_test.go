package website

import (
	"encoding/json"
	"errors"
	"io/ioutil"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gorilla/sessions"
	"gopkg.in/mgo.v2/bson"
)

func TestLogIntoTheServer(t *testing.T) {
	users, _ := newMockData()
	handler := LoginHandler(users)
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
	users, _ := newMockData()
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

func newMockData() (*MockData, Token) {
	d := &MockData{
		users: make(map[string]User),
	}

	_, _ = d.CreateUser("admin", "password1")
	tok, _ := d.LoginUser("admin", "password1")

	return d, tok
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

	return Token{Id: bson.NewObjectId(), User: user.Id}, nil
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

func (m *MockData) TokenIsValid(tok bson.ObjectId) bool {
	panic("Not Implemented")
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
