import React, { Component } from "react";
import { connect } from 'react-redux'
import {
  Route,
  BrowserRouter,
  Redirect,
} from "react-router-dom";
import Home from "./Home";
import Header from "./Header";
import Timesheets from "./Timesheets";
import Login from "./Login";
import Logout from "./Logout";
import Resume from "./Resume";
import Forbidden from "./Forbidden";

class Main extends Component {
  render() {
    const { username } = this.props;

    return (
      <BrowserRouter>
        <div>
          <Header username={this.props.username} />
          <div className="content">
            <Route exact path="/" component={Home}/>
            <Route path="/resume" component={Resume}/>
            <PrivateRoute authed={username} path="/timesheets" component={Timesheets} />
            <Route path="/login" component={Login}/> 
            <Route path="/logout" component={Logout}/> 
            <Route path="/forbidden" component={Forbidden}/> 
          </div>
        </div>
      </BrowserRouter>
    );
  }
}

const PrivateRoute = ({ component: Component, authed, ...rest }) => (
  <Route {...rest} render={props => (
    authed ? (
      <Component {...props}/>
    ) : (
      <Redirect to={{
        pathname: '/login',
        state: { from: props.location }
      }}/>
    )
  )}/>
);

function mapStateToProps(state) { 
  return {
    username: state.login.username
  };
}

function mapDispatchToProps(dispatch) { 
  return {};
}

export default connect(mapStateToProps, mapDispatchToProps)(Main);
