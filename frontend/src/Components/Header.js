import React, { Component } from "react";
import {
  NavLink,
  withRouter,
} from "react-router-dom";
import { connect } from "react-redux";
import {
  Navbar,
  NavbarToggler,
  Nav,
  NavItem,
  Collapse,
} from "reactstrap";
import { startLogout } from "../reducers";

class Header extends Component {
  constructor(props) {
    super(props);

    this.state = {
      isOpen: false,
    };

    this.onLogout = this.onLogout.bind(this);
    this.toggle = this.toggle.bind(this);
  }

  toggle() {
    this.setState({
      isOpen: !this.state.isOpen
    });
  }

  onLogout(e) {
    e.preventDefault();
    this.props.logout()
      .then(() => this.props.history.replace("/"));
  }

  render() {
    const username = this.props.username;

    return (
      <Navbar color="dark" dark expand="md">
        <NavLink className="navbar-brand" to="/">Michael-F-Bryan</NavLink>
        <NavbarToggler onClick={this.toggle} />
        <Collapse isOpen={this.state.isOpen} navbar>
          <Nav className="ml-auto" navbar>
            <NavItem>
              <NavLink className="nav-link" to="/resume">Resume</NavLink>
            </NavItem>
            <NavItem className={username ? "" : "d-none"}>
              <NavLink className="nav-link" to="/timesheets">Timesheets</NavLink>
            </NavItem>
            <NavItem className={username ? "d-none" : ""}>
              <NavLink className="nav-link" to="/login">Login</NavLink>
            </NavItem>
            <NavItem className={username ? "" : "d-none"}>
              <a className="nav-link" style={{cursor: "pointer"}} onClick={this.onLogout}>Log Out ({username})</a>
            </NavItem>
          </Nav>
        </Collapse>
      </Navbar>
    );
  }
}

function mapStateToProps(state) { 
  return { };
}

function mapDispatchToProps(dispatch) { 
  return {
    logout: () => dispatch(startLogout()),
  }
};

export default connect(mapStateToProps, mapDispatchToProps)(withRouter(Header));
