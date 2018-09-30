package website

import (
	"golang.org/x/crypto/bcrypt"
	"gopkg.in/mgo.v2/bson"
)

type User struct {
	Id           bson.ObjectId `bson:"_id,omitempty"`
	Name         string        `bson:"name"`
	passwordHash []byte        `bson:"hash"`
}

func NewUser(name, password string) (*User, error) {
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return nil, err
	}

	return &User{Id: bson.NewObjectId(), Name: name, passwordHash: hash}, nil
}

func (u *User) PasswordIsValid(password string) bool {
	return bcrypt.CompareHashAndPassword(u.passwordHash, []byte(password)) == nil
}