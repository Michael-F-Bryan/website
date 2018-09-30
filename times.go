package website

import (
	"errors"
	"time"

	"gopkg.in/mgo.v2/bson"
)

/// A single timesheet entry.
type Entry struct {
	ID        bson.ObjectId `bson:"_id,omitempty"`
	Start     time.Time     `bson:"start"`
	End       time.Time     `bson:"end"`
	Breaks    time.Duration `bson:"breaks"`
	Morning   string        `bson:"morning"`
	Afternoon string        `bson:"afternoon"`
}

func NewEntry(start, end time.Time) *Entry {
	return &Entry{
		Start: start,
		End:   end,
	}
}

func (e *Entry) TimeWorked() (time.Duration, error) {
	if e.Start.After(e.End) {
		return 0, errors.New("You can't start after you end")
	}

	duration := e.End.Sub(e.Start)
	if duration < e.Breaks {
		return 0, errors.New("End-Start must be longer than the Breaks")
	}

	return duration - e.Breaks, nil
}
