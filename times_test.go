package website

import (
	"testing"
	"time"

	"gopkg.in/mgo.v2/bson"
)

func TestGetTimeWorked(t *testing.T) {
	now := time.Now()
	workDay := 8 * time.Hour
	breaks := 30 * time.Minute

	entry := NewEntry(bson.NewObjectId(), now, now.Add(workDay))
	entry.Breaks = breaks

	got, err := entry.TimeWorked()
	if err != nil {
		t.Fatal(err)
	}

	shouldBe := workDay - breaks
	if got != shouldBe {
		t.Fatalf("expected %v but got %v", shouldBe, got)
	}
}
