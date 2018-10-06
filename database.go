package website

import (
	"errors"
	"time"

	"github.com/globalsign/mgo"
	"golang.org/x/crypto/bcrypt"
	"gopkg.in/mgo.v2/bson"
)

const VERSION string = "0.1.0"

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

	if err := session.Ping(); err != nil {
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

func (db *Database) Logout(tok bson.ObjectId) error {
	change := bson.M{"deleted": true, "last_seen": time.Now()}
	return db.inner.C("tokens").UpdateId(tok, bson.M{"$set": change})
}

func (db *Database) GetToken(tok bson.ObjectId) *Token {
	var got Token
	err := db.inner.C("tokens").FindId(tok).One(&got)
	if err == nil && !got.Deleted && time.Now().Sub(got.LastSeen) < TOKEN_TIMEOUT {
		return &got
	}

	return nil
}

func (db *Database) UpdateLastSeen(id bson.ObjectId, now time.Time) error {
	token := db.GetToken(id)

	if token == nil {
		return errors.New("Invalid token")
	}

	token.LastSeen = now
	_, err := db.inner.C("tokens").Upsert(bson.M{"_id": id}, token)
	return err
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

func (db *Database) GetUserById(id bson.ObjectId) (*User, error) {
	var user User
	err := db.inner.C("users").Find(bson.M{"_id": id}).One(&user)
	return &user, err
}

func (db *Database) GetEntryById(id bson.ObjectId) (Entry, error) {
	panic("Not Implemented")
}
func (db *Database) UpdateOrInsertTimesheet(entry Entry) error {
	panic("Not Implemented")
}
func (db *Database) DeleteTimesheet(entry Entry) error {
	panic("Not Implemented")
}
func (db *Database) NumTimesheets() (int, error) {
	panic("Not Implemented")
}
