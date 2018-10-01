import React, { Component } from "react";
import { connect } from "react-redux";
import { withRouter } from "react-router-dom";
import { startLogin } from "../reducers";

class Login extends Component {
  constructor(props) {
    super(props);

    this.state = {
      username: "",
      password: ""
    };

    this.handleChange = this.handleChange.bind(this);
  }

  handleChange(e) {
    const change = {};
    change[e.target.name] = e.target.value;
    this.setState(change);
  }

  render() {
    const { username, password } = this.state;
    const { history, onLogin } = this.props;
    const login = e => onLogin({e, username, password, history });

    return (
      <div id="login-container">
        <h1>Log In</h1>
        <form onSubmit={login}>
          <div className="login-row">
            <label htmlFor="username">Username</label>
            <input type="text" name="username" id="username" placeholder="Your Username" 
              value={username} onChange={this.handleChange} />
          </div>
          <div className="login-row">
            <label htmlFor="password">Password</label>
            <input type="password" name="password" id="password" 
              value={password} onChange={this.handleChange}  />
          </div>
          <button type="submit" className="login-submit">Submit</button>
        </form>
      </div>
    );
  }
}

function mapStateToProps(state) { 
  return { };
}

function mapDispatchToProps(dispatch) { 
  return {
    onLogin: ({ e, username, password, history }) => {
      console.log("Started logging in");
      e.preventDefault();

      var loginPromise = startLogin("/api", username, password);
      dispatch(d => loginPromise(d).then(() => history.replace("/")));
    },
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(withRouter(Login));
