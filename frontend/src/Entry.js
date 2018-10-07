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

  timeWorked() {
    return moment.duration(this.end.diff(this.start)).subtract(this.breaks);
  }

  toJSON() {
    const cleaned = Object.assign({}, this);
    cleaned.breaks = cleaned.breaks.asSeconds();
    return JSON.stringify(cleaned);
  }

  clone() {
    return new Entry(this.id, this.start, this.end, this.breaks, this.morning, this.afternoon);
  }
}

Entry.fromJSON = json => {
  return new Entry(json.id, json.start, json.end, json.breaks*1000, json.morning || "", json.afternoon || "")};
