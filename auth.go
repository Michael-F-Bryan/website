package main

import (
	"errors"
	"golang.org/x/crypto/bcrypt"
	"sync"
)

type Token int

type Auth struct {
	lock      sync.RWMutex
	lastToken Token
	users     map[string]*User
	tokens    map[Token]string
}

func NewAuth() *Auth {
	return &Auth{
		users:  make(map[string]*User),
		tokens: make(map[Token]string),
	}
}

func (a *Auth) LoginUser(username, password string) (Token, error) {
	a.lock.RLock()
	user, exists := a.users[username]
	a.lock.RUnlock()

	if !exists {
		return 0, errors.New("Invalid User")
	}

	if !user.PasswordIsValid(password) {
		return 0, errors.New("Incorrect Password")
	}

	return a.newToken(user.Name), nil
}

func (a *Auth) newToken(username string) Token {
	a.lock.Lock()
	defer a.lock.Unlock()

	a.lastToken += 1
	a.tokens[a.lastToken] = username

	return a.lastToken
}

type User struct {
	Name         string
	passwordHash []byte
}

func NewUser(name, password string) (*User, error) {
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return nil, err
	}

	return &User{Name: name, passwordHash: hash}, nil
}

func (u *User) PasswordIsValid(password string) bool {
	return bcrypt.CompareHashAndPassword(u.passwordHash, []byte(password)) != nil
}
