package website

import (
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"gopkg.in/mgo.v2/bson"
)

func TestLogIntoTheServer(t *testing.T) {
	users := newMockData()
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

	return Token{Id: bson.NewObjectId(), User: user.Id}, nil
}

func (m *MockData) GetUsers() ([]string, error) {
	panic("Not Implemented")
}

func (m *MockData) DeleteUser(username string) error {
	panic("Not Implemented")
}

func (m *MockData) Logout(tok Token) error {
	panic("Not Implemented")
}

func (m *MockData) TokenIsValid(tok Token) bool {
	panic("Not Implemented")
}

type MockTimes struct{}

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
