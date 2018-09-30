package website

import (
	"errors"
	"log"

	uuid "github.com/satori/go.uuid"
	"golang.org/x/crypto/bcrypt"
	"gopkg.in/mgo.v2"
	"gopkg.in/mgo.v2/bson"
)

type UserData interface {
	/// Create a new user.
	CreateUser(username, password string) (User, error)
	/// Log a user in, retrieving a unique login token
	LoginUser(username, password string) (Token, error)
	/// Log a user out, invalidating their login token
	Logout(tok Token) error
	/// Is the holder of this token allowed to access the website?
	TokenIsValid(tok Token) bool
}

type Token uuid.UUID

const DEFAULT_DATBASE string = "website"

var NilToken Token = Token(uuid.Nil)

type Database struct {
	inner *mgo.Database
}

func NewDatabaseFromMongo(db *mgo.Database) *Database {
	return &Database{inner: db}
}

func NewDatabase(url string) (*Database, error) {
	session, err := mgo.Dial(url)
	if err != nil {
		return nil, err
	}

	db := session.DB(DEFAULT_DATBASE)

	return NewDatabaseFromMongo(db), nil
}

func (db *Database) Close() error {
	db.inner.Session.Close()
	return nil
}

func (db *Database) CreateUser(username, password string) (User, error) {
	if existing, _ := db.GetUser(username); existing.Id != bson.ObjectId("") {
		return User{}, errors.New("User already exists")
	}

	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return User{}, err
	}

	user := User{Id: bson.NewObjectId(), Name: username, passwordHash: hash}

	if err = db.inner.C("users").Insert(&user); err != nil {
		return User{}, err
	}

	return user, nil
}

func (db *Database) LoginUser(username, password string) (Token, error) {
	user, err := db.GetUser(username)
	if err != nil {
		return NilToken, err
	}

	log.Println(user)

	if !user.PasswordIsValid(password) {
		return NilToken, errors.New("Incorrect Password")
	}

	tok, err := uuid.NewV4()
	if err != nil {
		return NilToken, err
	}

	err = db.InsertToken(Token(tok), &user)
	if err != nil {
		return NilToken, err
	}

	return Token(tok), nil
}

func (db *Database) Logout(tok Token) error {
	panic("Not Implemented")
}

func (db *Database) TokenIsValid(tok Token) bool {
	panic("Not Implemented")
}

func (db *Database) GetUser(username string) (User, error) {
	var user User
	err := db.inner.C("users").Find(bson.M{"name": username}).One(&user)
	return user, err
}

func (db *Database) InsertToken(tok Token, user *User) error {
	panic("Not Implemented")
}
