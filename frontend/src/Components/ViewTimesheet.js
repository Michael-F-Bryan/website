import React, { Component } from "react";
import { ButtonGroup, Button, Card, CardBody, CardHeader } from "reactstrap";
import { connect } from "react-redux";
import { FaPencilAlt as FaPencil, FaTrash } from "react-icons/fa";
import PropTypes from "prop-types";
import ReactMarkdown from "react-markdown";
import Entry from "../Entry";
import moment from "moment";
import { deleteTimesheetEntry } from "../reducers";

class ViewTimesheet extends Component {
  constructor(props) {
    super(props);

    this.state = {};

    this.dismissErrors = this.dismissErrors.bind(this);
    this.getEntry = this.getEntry.bind(this);
    this.onDelete = this.onDelete.bind(this);
  }

  dismissErrors() {
    this.setState({ error: null });
  }

  getEntry() {
    const { times } = this.props;
    const { id } = this.props.match.params;

    const found = times.find(entry => entry.id.toString() === id.toString());
    if (!found) {
      throw new Error(`Invalid timesheet entry id, ${id}`);
    }

    return found;
  }

  onDelete() {
    const entry = this.getEntry();
    this.props.delete(entry.id)
    .then(
        () => this.props.history.push("/timesheets"),
        error => this.setState({ error: error.toString() })
      );
  }

  render() {
    const entry = this.getEntry();
    const timeWorked = Math.round((entry.end - entry.start)*10)/10;

    const { error } = this.state;
    var errorMessage;

    if (error) {
      errorMessage = (
        <div className="alert alert-danger fade show my-2" role="alert">
          {error}
          <button type="button" className="close" aria-label="Close" onClick={this.dismissErrors}>
                <span aria-hidden="true">&times;</span>
          </button>
        </div>
      );
    }

    return (
      <div className="container">
        <h1 className="my-3">View Timesheet</h1>

        <ButtonGroup>
          <Button outline onClick={() => this.props.history.push(`/timesheets/${entry.id}/edit`)}><FaPencil/></Button>
          <Button outline onClick={this.onDelete}><FaTrash /></Button>
        </ButtonGroup>

        {errorMessage}

        <Card className="my-2">
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
        <Card className="my-4">
          <CardHeader>Morning</CardHeader>
          <CardBody>
            <ReactMarkdown source={entry.morning} />
          </CardBody>
        </Card>
        <Card className="my-4">
          <CardHeader>Afternoon</CardHeader>
          <CardBody>
            <ReactMarkdown source={entry.afternoon} />
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
  return {
    delete: id => dispatch(deleteTimesheetEntry("/api", id))
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(ViewTimesheet);
