package website

import (
	"errors"
	"time"

	"gopkg.in/mgo.v2/bson"
)

type Timesheets interface {
	GetEntryById(id bson.ObjectId) (Entry, error)
	UpdateOrInsertTimesheet(entry Entry) error
	DeleteTimesheet(id bson.ObjectId) error
	GetEntries(userId bson.ObjectId, start, end time.Time) ([]Entry, error)
}

// A single timesheet entry.
type Entry struct {
	ID        bson.ObjectId `bson:"_id,omitempty"`
	User      bson.ObjectId `bson:"user"`
	Start     time.Time     `bson:"start"`
	End       time.Time     `bson:"end"`
	Breaks    time.Duration `bson:"breaks"`
	Morning   string        `bson:"morning"`
	Afternoon string        `bson:"afternoon"`
}

func NewEntry(user bson.ObjectId, start, end time.Time) Entry {
	return Entry{
		ID:    bson.NewObjectId(),
		User:  user,
		Start: start,
		End:   end,
	}
}

func (e Entry) TimeWorked() (time.Duration, error) {
	if e.Start.After(e.End) {
		return 0, errors.New("You can't start after you end")
	}

	duration := e.End.Sub(e.Start)
	if duration < e.Breaks {
		return 0, errors.New("End-Start must be longer than the Breaks")
	}

	return duration - e.Breaks, nil
}

func (e Entry) Equals(other Entry) bool {
	return e.ID == other.ID && e.User == other.User
}
