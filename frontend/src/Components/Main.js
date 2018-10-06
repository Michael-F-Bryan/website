import React, { Component } from "react";
import { connect } from 'react-redux'
import {
  BrowserRouter,
  Redirect,
  Route,
  Switch,
} from "react-router-dom";
import Home from "./Home";
import Header from "./Header";
import Timesheets from "./Timesheets";
import EditTimesheet from "./EditTimesheet";
import Login from "./Login";
import Logout from "./Logout";
import Resume from "./Resume";
import RouteChanged from "./RouteChanged";
import Forbidden from "./Forbidden";
import { ping } from "../reducers";

class Main extends Component {
  componentDidMount() {
    // We should ping the server on load
    this.props.onRefresh()
  }

  render() {
    const { username } = this.props;

    return (
      <BrowserRouter>
        <div>
          <RouteChanged onRouteChanged={this.props.onRefresh} />
          <Header username={this.props.username} />
          <div className="content">
            <Switch>
              <Route exact path="/" component={Home}/>
              <Route path="/resume" component={Resume}/>
              <PrivateRoute authed={username} path="/timesheets/new" component={EditTimesheet} />
              <PrivateRoute authed={username} path="/timesheets/:id" component={EditTimesheet} />
              <PrivateRoute authed={username} path="/timesheets/" component={Timesheets} />
              <Route path="/login" component={Login}/> 
              <Route path="/logout" component={Logout}/> 
              <Route path="/forbidden" component={Forbidden}/> 
            </Switch>
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
  return {
    onRefresh: () => {
      dispatch(ping("/api"));
    }
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(Main);
