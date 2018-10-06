import React, { Component } from "react";
import { Card, CardHeader, CardBody, CardText } from "reactstrap";

export default class Summary extends Component {
  render() {
    return (
      <Card className="my-4">
        <CardHeader data-toggle="collapse" data-target="#summary-body" aria-expanded="false" aria-controls="summary-body"> 
          Summary
        </CardHeader>
        <CardBody id="summary-body" className="collapse show">
            <CardText>With supporting text below as a natural lead-in to additional content.</CardText>
          </CardBody>
      </Card>
    );
  }
}
