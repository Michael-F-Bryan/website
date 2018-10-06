import React, { Component } from "react";
import { connect } from "react-redux";
import { Button, ButtonGroup, Table } from "reactstrap";
import Summary from "./Summary";

const periods = [
  { days: -1, label: "All Time" },
  { days: 7, label: "Week" },
  { days: 31, label: "Month" },
  { days: Math.round(365/4), label: "Quarter" },
  { days: 365, label: "Year" },
];
 
class Timesheets extends Component {
  constructor(props) {
    super(props);

    this.state = { days: -1 };
  }

  setDays(days) {
    this.setState({ days });
  }

  render() {
    const buttons = periods.map(period => {
      const { days, label } = period;
      const isActive = this.state.days === days;

      return (
        <Button key={label} outline className={isActive ? "active" : ""}
                onClick={() => this.setDays(days)}>
          {label}
        </Button>
      );
    });

    var times = this.props.times;

    if(this.state.days > 0) {
      const cutoff = new Date();
      cutoff.setDate(cutoff.getDate() - this.state.days);
      console.log(cutoff);
      times = times.filter(time => cutoff < time.start);
    } 

    const rows = times.map((time, i) => {
      const hoursWorked = (time.end - time.start - time.breaks)/1000/60/60;
      return (
        <tr key={i}>
          <td>{i+1}</td>
          <td>{time.start.toLocaleDateString()}</td>
          <td>{time.start.toLocaleTimeString()}</td>
          <td>{time.end.toLocaleTimeString()}</td>
          <td>{Math.round(hoursWorked*10)/10}</td>
          <td></td>
        </tr>
      );
    });

    return (
      <div className="container">
        <h2 className="my-3">Timesheets</h2>

        <ButtonGroup className="my-2">{buttons}</ButtonGroup>
        <Summary times={times} />

        <Table hover>
          <thead>
            <tr>
              <th>#</th>
              <th>Date</th>
              <th>Start</th>
              <th>End</th>
              <th>HoursWorked</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {rows}
          </tbody>
        </Table>
      </div>
    );
  }
}


function mapStateToProps(state) { 
  return { times: state.times };
}

function mapDispatchToProps(dispatch) { 
  return {};
}

export default connect(mapStateToProps, mapDispatchToProps)(Timesheets);
