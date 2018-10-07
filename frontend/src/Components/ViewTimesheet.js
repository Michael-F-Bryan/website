import React, { Component } from "react";
import { Card, CardBody } from "reactstrap";
import { connect } from "react-redux";
import PropTypes from "prop-types";
import Entry from "../Entry";
import moment from "moment";

class ViewTimesheet extends Component {
  getEntry() {
    const { times } = this.props;
    const { id } = this.props.match.params;

    for(const entry of times) {
      if (String(entry.id) === id) {
        return entry;
      }
    }

    throw new Error(`Invalid timesheet entry id, ${id}`);
  }

  render() {
    const entry = this.getEntry();
    const timeWorked = Math.round((entry.end - entry.start)*10)/10;

    return (
      <div className="container">
        <h1>View Timesheet</h1>

        <Card className="my-4">
          <CardBody>
            <dl className="row">
              <dt className="col-sm-3">Date</dt>
              <dd className="col-sm-9">{entry.start.format("LL")}</dd>

              <dt className="col-sm-3">Start</dt>
              <dd className="col-sm-9">{entry.start.format("LT")}</dd>

              <dt className="col-sm-3">End</dt>
              <dd className="col-sm-9">{entry.end.format("LT")}</dd>

              <dt className="col-sm-3">Breaks</dt>
              <dd className="col-sm-9">{moment.duration(entry.breaks).humanize()}</dd>

              <dt className="col-sm-3">Hours Worked</dt>
              <dd className="col-sm-9">{moment.duration(timeWorked).humanize()}</dd>
            </dl>
          </CardBody>
        </Card>
      </div>
    );
  }
}

ViewTimesheet.propTypes = {
  times: PropTypes.arrayOf(Entry).isRequired
};

function mapStateToProps(state) { 
  return { times: state.times };
}

function mapDispatchToProps(dispatch) { 
  return {};
}

export default connect(mapStateToProps, mapDispatchToProps)(ViewTimesheet);
