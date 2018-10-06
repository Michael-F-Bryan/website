package website

import (
	"encoding/hex"
	"time"

	"golang.org/x/crypto/bcrypt"
	"gopkg.in/mgo.v2/bson"
)

type UserData interface {
	// Create a new user.
	CreateUser(username, password string) (User, error)
	// Log a user in, retrieving a unique login token
	LoginUser(username, password string) (Token, error)
	GetUsers() ([]string, error)
	DeleteUser(username string) error
	// Log a user out, invalidating their login token
	Logout(tokenId bson.ObjectId) error
	// Get the corresponding Token from its ID, returning nil if it's no longer
	// valid.
	GetToken(tok bson.ObjectId) *Token
}

type Token struct {
	Id       bson.ObjectId `bson:"_id,omitempty"`
	User     bson.ObjectId `bson:"user_id"`
	Created  time.Time     `bson:"created"`
	LastSeen time.Time     `bson:"last_seen"`
	Deleted  bool          `bson:"deleted"`
}

func (t Token) String() string {
	repr := hex.EncodeToString([]byte(t.Id))
	if t.Deleted {
		repr += " (deleted)"
	}

	return repr
}

const DEFAULT_DATBASE string = "website"

var TOKEN_TIMEOUT time.Duration = 7 * 24 * time.Hour

var NilToken Token = Token{}

type User struct {
	Id           bson.ObjectId `bson:"_id,omitempty"`
	Name         string        `bson:"name"`
	PasswordHash []byte        `bson:"password"`
}

func NewUser(name, password string) (User, error) {
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil {
		return User{}, err
	}

	return User{Id: bson.NewObjectId(), Name: name, PasswordHash: hash}, nil
}

func (u *User) PasswordIsValid(password string) bool {
	return bcrypt.CompareHashAndPassword(u.PasswordHash, []byte(password)) == nil
}
