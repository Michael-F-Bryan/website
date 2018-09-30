package website

import (
	"errors"
	"time"

	"github.com/globalsign/mgo"
	"golang.org/x/crypto/bcrypt"
	"gopkg.in/mgo.v2/bson"
)

type UserData interface {
	/// Create a new user.
	CreateUser(username, password string) (User, error)
	/// Log a user in, retrieving a unique login token
	LoginUser(username, password string) (Token, error)
	GetUsers() ([]string, error)
	DeleteUser(username string) error
	/// Log a user out, invalidating their login token
	Logout(tok Token) error
	/// Is the holder of this token allowed to access the website?
	TokenIsValid(tok Token) bool
}

type Token struct {
	Id       bson.ObjectId `bson:"_id,omitempty"`
	User     bson.ObjectId `bson:"user_id"`
	Created  time.Time     `bson:"created"`
	LastSeen time.Time     `bson:"last_seen"`
	Deleted  bool          `bson:"deleted"`
}

const DEFAULT_DATBASE string = "website"

var TOKEN_TIMEOUT time.Duration = 7 * 24 * time.Hour

var NilToken Token = Token{}

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

	user := User{Id: bson.NewObjectId(), Name: username, PasswordHash: hash}

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

	if !user.PasswordIsValid(password) {
		return NilToken, errors.New("Incorrect Password")
	}

	now := time.Now()
	tok := Token{
		Id:       bson.NewObjectId(),
		User:     user.Id,
		Created:  now,
		LastSeen: now,
	}

	err = db.inner.C("tokens").Insert(&tok)
	if err != nil {
		return NilToken, err
	}

	return tok, nil
}

func (db *Database) Logout(tok Token) error {
	change := bson.M{"deleted": true, "last_seen": time.Now()}
	return db.inner.C("tokens").UpdateId(tok.Id, bson.M{"$set": change})
}

func (db *Database) TokenIsValid(tok Token) bool {
	var got Token
	err := db.inner.C("tokens").FindId(tok.Id).One(&got)
	if err != nil {
		return false
	}

	return !got.Deleted && time.Now().Sub(got.LastSeen) < TOKEN_TIMEOUT
}

func (db *Database) GetUser(username string) (User, error) {
	var user User
	err := db.inner.C("users").Find(bson.M{"name": username}).One(&user)
	return user, err
}

func (db *Database) DeleteUser(username string) error {
	return db.inner.C("users").Remove(bson.M{"name": username})
}

func (db *Database) GetUsers() ([]string, error) {
	iter := db.inner.C("users").Find(nil).Iter()

	var users []string
	var user User

	for iter.Next(&user) && !iter.Timeout() {
		users = append(users, user.Name)
	}

	if iter.Timeout() {
		return nil, errors.New("Timed out")
	}
	if err := iter.Close(); err != nil {
		return nil, err
	}

	return users, nil
}
