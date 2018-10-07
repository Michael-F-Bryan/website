package website

import (
	"fmt"
	"sort"
	"testing"
	"time"

	"github.com/globalsign/mgo"
	uuid "github.com/satori/go.uuid"
	"gopkg.in/mgo.v2/bson"
)

const port int = 27017

func temporaryDatabase(t *testing.T) (*Database, func(), error) {
	session, err := mgo.DialWithTimeout(fmt.Sprintf("localhost:%d", port), 1*time.Second)
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
	requiresUserData := func(_ UserData) {}

	requiresUserData(&Database{})
}

func TestDatabaseImplementsTimesheets(t *testing.T) {
	requiresUserData := func(_ Timesheets) {}

	requiresUserData(&Database{})
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

	if token == NilToken {
		t.Error("Got the nil token")
	}

	if tok := db.GetToken(token.Id); tok == nil {
		t.Error("The token isn't valid")
	}

	err = db.Logout(token.Id)
	if err != nil {
		t.Errorf("Couldn't logout, %v", err)
	}

	if tok := db.GetToken(token.Id); tok != nil {
		t.Error("The token should be invalid")
	}
}

func TestGetEntriesWithinRange(t *testing.T) {
	db, closer, err := temporaryDatabase(t)
	if err != nil {
		t.Log(err)
		t.Skip("Can't connect to the database")
		return
	}
	defer closer()

	// set up the database with a couple entries
	user := bson.NewObjectId()
	now := time.Now().Truncate(time.Second)
	entries := []Entry{
		NewEntry(user, now.Add(-24*time.Hour), now.Add(-24*time.Hour)),
		NewEntry(user, now, now),
		NewEntry(bson.NewObjectId(), now, now),
		NewEntry(user, now.Add(24*time.Hour), now.Add(24*time.Hour)),
		NewEntry(user, now.Add(5*24*time.Hour), now.Add(5*24*time.Hour)),
	}
	for _, entry := range entries {
		if err := db.UpdateOrInsertTimesheet(entry); err != nil {
			t.Fatal(err)
		}
	}

	start := now.Add(-1 * time.Hour)
	end := now.Add(2 * 24 * time.Hour)

	got, err := db.GetEntries(user, start, end)
	if err != nil {
		t.Fatal(err)
	}

	shouldBe := []Entry{
		entries[1],
		entries[3],
	}
	if len(got) != len(shouldBe) {
		t.Errorf("Expected %d but found %d", len(shouldBe), len(got))
	}

	sorter := func(l, r int) bool { return got[l].Start.Before(got[r].Start) }
	sort.Slice(got, sorter)
	sort.Slice(shouldBe, sorter)

	for i, expected := range shouldBe {
		found := got[i]
		if !expected.Equals(found) {
			t.Errorf("at index %d, expected %#v but found %#v", i, expected, found)
		}
	}
}

func TestCreateEntryThenDeleteIt(t *testing.T) {
	db, closer, err := temporaryDatabase(t)
	if err != nil {
		t.Log(err)
		t.Skip("Can't connect to the database")
		return
	}
	defer closer()

	user := bson.NewObjectId()
	now := time.Now().Round(0)
	entry := NewEntry(user, now, now.Add(8*time.Hour))

	// create a new timesheet
	if err := db.UpdateOrInsertTimesheet(entry); err != nil {
		t.Fatal(err)
	}

	if _, err := db.GetEntryById(entry.ID); err != nil {
		t.Fatal(err)
	}

	// then delete it
	if err := db.DeleteTimesheet(entry.ID); err != nil {
		t.Fatal(err)
	}

	if _, err := db.GetEntryById(entry.ID); err == nil {
		t.Fatal("We should have encountered an error fetching the entry")
	}
}
