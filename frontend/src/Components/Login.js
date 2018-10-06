import React, { Component } from "react";
import { connect } from "react-redux";
import { withRouter, Redirect } from "react-router-dom";
import { startLogin, CLEAR_LOGIN_ERROR } from "../reducers";

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

    const { login_state, username, error } = this.props.login;

    if (login_state === "done" && username && !error) {
      const { from } = this.props.location.state || { from: { pathname: '/' } }
      return (
        <Redirect to={from} />
      );
    }

    const state = this.state;

    var errorMessage;

    if (error) {
      errorMessage = (
        <div className="alert alert-light fade show" role="alert">
          {error}
          <button type="button" className="close" aria-label="Close" onClick={this.props.dismissErrors}>
                <span aria-hidden="true">&times;</span>
          </button>
        </div>
      );
    }

    const onSubmit = e => this.props.onLogin({e, username: state.username, password: state.password });
    const form = (
        <form onSubmit={onSubmit}>
          <div className="login-row">
            <label htmlFor="username">Username</label>
            <input type="text" name="username" id="username" placeholder="Your Username" 
              value={state.username} onChange={this.handleChange} />
          </div>
          <div className="login-row">
            <label htmlFor="password">Password</label>
            <input type="password" name="password" id="password" 
              value={state.password} onChange={this.handleChange}  />
          </div>
          <button type="submit" className="login-submit">Submit</button>
        </form>
    );

    return (
      <div id="login-container">
        <h1>Log In</h1>
        {errorMessage}
        {form}
      </div>
    );
  }
}

function mapStateToProps(state) { 
  return { login: state.login };
}

function mapDispatchToProps(dispatch) { 
  return {
    onLogin: ({ e, username, password }) => {
      console.log("Started logging in");
      e.preventDefault();

      dispatch(startLogin("/api", username, password));
    },

    dismissErrors: () => dispatch({ type: CLEAR_LOGIN_ERROR })
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(withRouter(Login));
