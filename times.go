package website

import (
	"encoding/json"
	"errors"
	"math"
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
	ID        bson.ObjectId `bson:"_id,omitempty",json:"id,omitempty"`
	User      bson.ObjectId `bson:"user",json:"user,omitempty"`
	Start     time.Time     `bson:"start",json:"start,omitempty"`
	End       time.Time     `bson:"end",json:"end,omitempty"`
	Breaks    time.Duration `bson:"breaks",json:"breaks,omitempty"`
	Morning   string        `bson:"morning",json:"morning,omitempty"`
	Afternoon string        `bson:"afternoon",json:"afternoon,omitempty"`
}

func NewEntry(user bson.ObjectId, start, end time.Time) Entry {
	return Entry{
		ID:    bson.NewObjectId(),
		User:  user,
		Start: start,
		End:   end,
	}
}

func (e *Entry) UnmarshalJSON(data []byte) error {
	var temp marshalEntry

	if err := json.Unmarshal(data, &temp); err != nil {
		return nil
	}

	e.updateFrom(temp)

	return nil
}

func (e *Entry) MarshalJSON() ([]byte, error) {
	var temp marshalEntry
	temp.updateFrom(*e)

	return json.Marshal(&temp)
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

type marshalEntry struct {
	ID        bson.ObjectId `json:"id,omitempty"`
	User      bson.ObjectId `json:"user,omitempty"`
	Start     time.Time     `json:"start,omitempty"`
	End       time.Time     `json:"end,omitempty"`
	Breaks    float64       `json:"breaks,omitempty"`
	Morning   string        `json:"morning,omitempty"`
	Afternoon string        `json:"afternoon,omitempty"`
}

func (m *marshalEntry) updateFrom(e Entry) {
	m.ID = e.ID
	m.User = e.User
	m.Start = e.Start
	m.End = e.End
	m.Morning = e.Morning
	m.Afternoon = e.Afternoon

	m.Breaks = e.Breaks.Seconds()
}

func (e *Entry) updateFrom(temp marshalEntry) {
	e.ID = temp.ID
	e.User = temp.User
	e.Start = temp.Start
	e.End = temp.End
	e.Morning = temp.Morning
	e.Afternoon = temp.Afternoon

	secs := int(math.Floor(temp.Breaks))
	fract := temp.Breaks - math.Floor(temp.Breaks)
	nanos := int(math.Round(fract * 1000 * 1000 * 1000))
	e.Breaks = time.Duration(secs)*time.Second + time.Duration(nanos)*time.Nanosecond
}
