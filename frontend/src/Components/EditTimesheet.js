import React, { Component } from "react";
import { connect } from "react-redux";
import { Button, ButtonGroup, Col, Form, FormGroup, Label, Input } from "reactstrap";
import { withRouter } from "react-router-dom";
import moment from "moment";
import Entry from "../Entry";
import { saveTimesheetEntry } from "../reducers";

class EditTimesheet extends Component {
  constructor(props) {
    super(props);

    // make sure we initialize the state properly, whether we're creating
    // a new entry or just editing an old one
    if (props.match.params.id) {
      const { id } = props.match.params;
      const entry = props.times.find(entry => entry.id.toString() === id);
      this.state = Object.assign({ day: entry.start }, entry);
    } else {
      this.state = {
        day: moment(),
        start: moment(),
        end: moment().add(8, "hours"),
        breaks: moment.duration(0),
      };
    }

    // then convert fields so they'll be accepted by the relevant input
    // component
    this.state.day = this.state.day.format(moment.HTML5_FMT.DATE);
    this.state.start = this.state.start.format(moment.HTML5_FMT.TIME);
    this.state.end = this.state.end.format(moment.HTML5_FMT.TIME);
    this.state.breaks = this.state.breaks.asMinutes();

    this.handleChange = this.handleChange.bind(this);
    this.onSubmit = this.onSubmit.bind(this);
    this.dismissErrors = this.dismissErrors.bind(this);
  }

  onSubmit(e) {
    e.preventDefault();

    var { id, day, start, end, breaks, morning, afternoon } = this.state;

    day = moment(day, moment.HTML5_FMT.DATE);
    start = moment.duration(start, moment.HTML5_FMT.TIME);
    start = moment(day).add(start);
    end = moment.duration(end, moment.HTML5_FMT.TIME);
    end = moment(day).add(end);

    const entry = new Entry(id, start, end, breaks * 1000, morning, afternoon)
    try {
      entry.validate();
    } catch (e) {
      this.setState({ error: e.toString() });
      return;
    }

    this.props.onSubmit(entry)
    .then(
      outcome => {
        const { id } = outcome;
        this.props.history.push("/timesheets/" + id);
      },
      error => {
        if (error instanceof Error) {
          // it's some sort of exception
          this.setState({ error: error.toString() });
        } else {
          // assume it's an unsuccessful JSON response
          this.setState({ error: error.error });
        }
      }
    );
  }

  handleChange(e) {
    const change = {};
    change[e.target.name] = e.target.value;
    this.setState(change);
  }

  dismissErrors() {
    this.setState({ error: null });
  }

  render() {
    const { error, id } = this.state;

    var errorMessage;

    if (error) {
      errorMessage = (
        <div className="alert alert-danger fade show" role="alert">
          {error}
          <button type="button" className="close" aria-label="Close" onClick={this.dismissErrors}>
                <span aria-hidden="true">&times;</span>
          </button>
        </div>
      );
    }

    return (
      <div className="container">
        <h1 className="my-3">{id ? "Edit Timesheet" : "New Timesheet"}</h1>
        {errorMessage}
        <Form onSubmit={this.onSubmit}>
          <FormGroup row>
            <Label for="day" sm={2}>Day</Label>
            <Col sm={10}>
              <Input type="date" name="day" value={this.state.day} onChange={this.handleChange} />
            </Col>
          </FormGroup>
          <FormGroup row>
            <Label for="start" sm={2}>Start</Label>
            <Col sm={10}>
              <Input type="time" name="start" value={this.state.start} onChange={this.handleChange} />
            </Col>
          </FormGroup>
          <FormGroup row>
            <Label for="end" sm={2}>End</Label>
            <Col sm={10}>
              <Input type="time" name="end" value={this.state.end} onChange={this.handleChange} />
            </Col>
          </FormGroup>
          <FormGroup row>
            <Label for="breaks" sm={2}>Breaks</Label>
            <Col sm={10}>
              <Input type="number" name="breaks" value={this.state.breaks} onChange={this.handleChange} min="0" />
            </Col>
          </FormGroup>
          <FormGroup row>
            <Label for="morning" sm={2}>Morning</Label>
            <Col sm={10}>
              <Input type="textarea" name="morning" value={this.state.morning} onChange={this.handleChange} min="0" />
            </Col>
            </FormGroup>
            <FormGroup row>
            <Label for="afternoon" sm={2}>Afternoon</Label>
            <Col sm={10}>
              <Input type="textarea" name="afternoon" value={this.state.afternoon} onChange={this.handleChange} min="0" />
            </Col>
          </FormGroup>

          <ButtonGroup>
          <Button>Save</Button>
          <Button onClick={() => this.props.history.goBack()}>Cancel</Button>
        </ButtonGroup>
        </Form>
      </div>
    );
  }
}

function mapStateToProps(state) { 
  return { times: state.times };
}

function mapDispatchToProps(dispatch) { 
  return {
    onSubmit: entry => dispatch(saveTimesheetEntry("/api", entry))
  }; 
}

export default connect(mapStateToProps, mapDispatchToProps)(withRouter(EditTimesheet));
