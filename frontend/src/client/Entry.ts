import moment, { Moment, Duration, duration } from 'moment';

export default class Entry {
    public readonly id: string;
    public start: Moment;
    public end?: Moment;
    public description: string;
    public breaks: Duration;

    constructor(id: string, start: Date, end?: Date, description: string = '', breaks: number = 0) {
        this.id = id;
        this.start = moment(start);
        this.end = end !== undefined ? moment(end) : undefined;
        this.description = description;
        this.breaks = duration(breaks, 'minutes');
    }

    get timeWorked(): Duration {
        if (this.end !== undefined) {
            return duration(this.end.diff(this.start)).subtract(this.breaks);
        } else {
            return duration();
        }
    }
}
