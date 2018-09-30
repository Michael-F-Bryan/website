import React, { Component } from "react";
import {
  Route,
  NavLink,
  BrowserRouter
} from "react-router-dom";
import {
  Navbar,
  NavbarToggler,
  Nav,
  NavItem,
  Collapse,
} from "reactstrap";
import Home from "./Home";
import Timesheets from "./Timesheets";
import Login from "./Login";
import Logout from "./Logout";
import Resume from "./Resume";
import Forbidden from "./Forbidden";
import "../index.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import store from "../store";

export default class Main extends Component {
  constructor(props) {
    super(props);

    this.toggle = this.toggle.bind(this);
    this.state = {
      isOpen: false,
    };
  }

  toggle() {
    this.setState({
      isOpen: !this.state.isOpen
    });
  }

  render() {
    const { username } = store.getState();
    const hasUser = username && username !== "";

    return (
      <BrowserRouter>
        <div>
          <Navbar color="dark" dark expand="md">
            <NavLink className="navbar-brand" to="/">Michael-F-Bryan</NavLink>
            <NavbarToggler onClick={this.toggle} />
            <Collapse isOpen={this.state.isOpen} navbar>
              <Nav className="ml-auto" navbar>
                <NavItem>
                  <NavLink className="nav-link" to="/resume">Resume</NavLink>
                </NavItem>
                <NavItem className={hasUser ? "" : "d-none"}>
                  <NavLink className="nav-link" to="/timesheets">Timesheets</NavLink>
                </NavItem>
                <NavItem className={hasUser ? "d-none" : ""}>
                  <NavLink className="nav-link" to="/login">Login</NavLink>
                </NavItem>
                <NavItem className={hasUser ? "" : "d-none"}>
                  <NavLink className="nav-link" to="/logout">Log Out ({username})</NavLink>
                </NavItem>
              </Nav>
            </Collapse>
          </Navbar>
          <div className="content">
            <Route exact path="/" component={Home}/>
            <Route path="/resume" component={Resume}/>
            <Route path="/timesheets" component={Timesheets}/> 
            <Route path="/login" component={Login}/> 
            <Route path="/logout" component={Logout}/> 
            <Route path="/forbidden" component={Forbidden}/> 
          </div>
        </div>
      </BrowserRouter>
    );
  }
}
