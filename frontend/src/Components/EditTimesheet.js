import React, { Component } from "react";
import { connect } from "react-redux";
import { Redirect } from "react-router-dom";
import { Button, Col, Form, FormGroup, Label, Input } from "reactstrap";
import moment from "moment";
import { saveTimesheetEntry } from "../reducers";

class EditTimesheet extends Component {
  constructor(props) {
    super(props);

    this.state = {
      day: moment().format(moment.HTML5_FMT.DATE),
      start: moment().format(moment.HTML5_FMT.TIME),
      end: moment().add(8, "hours").format(moment.HTML5_FMT.TIME),
      breaks: 0,
      id: this.props.match.id,
    };

    this.state = Object.assign(this.state, props.entry || {});

    this.handleChange = this.handleChange.bind(this);
    this.onSubmit = this.onSubmit.bind(this);
    this.dismissErrors = this.dismissErrors.bind(this);
  }

  onSubmit(e) {
    e.preventDefault();

    this.props.onSubmit(this.state, 
      outcome => this.setState({ saved: true }),
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
    const { saved, error, id } = this.state;

    if (saved) {
      return (
        <Redirect to="/timesheets" />
      );
    }

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
        <h1>{id ? "Edit Timesheet" : "New Timesheet"}</h1>
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
          <Button>Save</Button>
        </Form>
      </div>
    );
  }
}

function mapStateToProps(state) { 
  return {};
}

function mapDispatchToProps(dispatch) { 
  return {
    onSubmit: (entry, success, error) => {
      dispatch(saveTimesheetEntry("/api", entry, success, error));
    }
  }; 
}

export default connect(mapStateToProps, mapDispatchToProps)(EditTimesheet);
