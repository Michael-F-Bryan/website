import React, { Component } from "react";
import { Card, CardHeader, CardBody } from "reactstrap";
import PropTypes from "prop-types";
import Entry from "../Entry";

export default class Summary extends Component {
  render() {
    const stats = calculateStats(this.props.times);

    return (
      <Card className="my-4">
        <CardHeader data-toggle="collapse" data-target="#summary-body" aria-expanded="false" aria-controls="summary-body"> 
          Summary
        </CardHeader>
        <CardBody id="summary-body" className="collapse show">
          <dl className="row">
            <dt className="col-sm-3">Total Days</dt>
            <dd className="col-sm-9">{stats.totalDays}</dd>

            <dt className="col-sm-3">Total Hours</dt>
            <dd className="col-sm-9">{Math.round(stats.totalHours*10)/10}</dd>

            <dt className="col-sm-3">Average Work Day</dt>
            <dd className="col-sm-9">{Math.round(stats.averageDay*10)/10}</dd>
          </dl>
        </CardBody>
      </Card>
    );
  }
}

Summary.propTypes = {
  times: PropTypes.arrayOf(PropTypes.instanceOf(Entry)).isRequired
};

function hoursWorked(start, end, breaks) {
    return (end - start - breaks)/1000/60/60;
}

function calculateStats(times) {
  const totalDays = times.length;
  const totalHours = times.map(time => hoursWorked(time.start, time.end, time.breaks))
    .reduce((acc, elem) => acc + elem, 0);
  const averageDay = totalHours/totalDays;

  return { totalDays, totalHours, averageDay };
}
