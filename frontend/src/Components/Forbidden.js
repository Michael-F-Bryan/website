import React, { Component } from "react";
import { NavLink } from "react-router-dom";
 
export default class Forbidden extends Component {
  render() {
    return (
      <div>
        <h2>Forbidden</h2>
        <p>It looks like you need to <NavLink to="/login">log in</NavLink> to that this page.</p>
      </div>
    );
  }
}


