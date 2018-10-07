import moment from "moment";

export default class Entry {
  constructor(id, start, end, breaks=0, morning="", afternoon="") {
    this.id = id;
    this.start = moment(start);
    this.end = moment(end);
    this.breaks = moment.duration(breaks);
    this.morning = morning;
    this.afternoon = afternoon;
  }

  validate() {
    if (this.start.isAfter(this.end)) {
      throw new Error("You can't end before you've started");
    }
    if (this.breaks < 0) {
      throw new Error("You can't have negative breaks");
    }
  }

  hoursWorked() {
    return moment.duration(this.end.diff(this.start)).subtract(this.breaks);
  }
}
