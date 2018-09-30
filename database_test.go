package website

import (
	"testing"
	"time"

	uuid "github.com/satori/go.uuid"
	mgo "gopkg.in/mgo.v2"
)

func temporaryDatabase(t *testing.T) (*Database, func(), error) {
	session, err := mgo.DialWithTimeout("localhost:32769", 1*time.Second)
	if err != nil {
		return nil, nil, err
	}

	name := uuid.Must(uuid.NewV4()).String()
	inner := session.DB(name)

	closer := func() {
		if err := inner.DropDatabase(); err != nil {
			t.Fatalf("Unable to drop the database, %v", err)
		}
		inner.Session.Close()
	}
	return NewDatabaseFromMongo(inner), closer, nil
}

func TestDatabaseImplementsUserData(t *testing.T) {
	db := &Database{}

	requiresUserData := func(_ UserData) {}

	requiresUserData(db)
}

func TestTypicalUserSession(t *testing.T) {
	db, closer, err := temporaryDatabase(t)
	if err != nil {
		t.Log(err)
		t.Skip("Can't connect to the database")
		return
	}
	defer closer()

	user, err := db.CreateUser("admin", "password1")
	if err != nil {
		t.Fatalf("Unable to create the user, %v", err)
	}
	if user.Name != "admin" {
		t.Errorf("Expected the user's name to be \"admin\", found %v", user.Name)
	}

	token, err := db.LoginUser("admin", "password1")
	if err != nil {
		t.Errorf("Couldn't login, %v", err)
	}

	if token != NilToken {
		t.Error("Got the nil token")
	}

	err = db.Logout(token)
	if err != nil {
		t.Errorf("Couldn't logout, %v", err)
	}
}
