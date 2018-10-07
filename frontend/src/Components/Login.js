import React, { Component } from "react";
import { connect } from "react-redux";
import { withRouter, Redirect } from "react-router-dom";
import { startLogin } from "../reducers";

class Login extends Component {
  constructor(props) {
    super(props);

    this.state = {
      username: "",
      password: ""
    };

    this.handleChange = this.handleChange.bind(this);
    this.onSubmit = this.onSubmit.bind(this);
  }

  handleChange(e) {
    const change = {};
    change[e.target.name] = e.target.value;
    this.setState(change);
  }

  onSubmit(e) {
    e.preventDefault();
    const { from } = this.props.location.state || { from: { pathname: "/" } };

    this.props.onLogin({e, username: this.state.username, password: this.state.password })
      .then(
        () => this.props.history.push(from),
        error => this.setState({ error: error.toString() })
      )
    ;
  }

  render() {
    const { error } = this.state;

    if (this.props.username) {
      const { from } = this.props.location.state || { from: { pathname: "/" } };
      return (
        <Redirect to={from} />
      );
    }

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

    const form = (
      <form onSubmit={this.onSubmit}>
        <div className="login-row">
          <label htmlFor="username">Username</label>
          <input type="text" name="username" id="username" placeholder="Your Username" 
            value={this.state.username} onChange={this.handleChange} />
        </div>
        <div className="login-row">
          <label htmlFor="password">Password</label>
          <input type="password" name="password" id="password" 
            value={this.state.password} onChange={this.handleChange}  />
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
  return { username: state.username };
}

function mapDispatchToProps(dispatch) { 
  return {
    onLogin: ({ e, username, password }) => dispatch(startLogin(username, password)),
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(withRouter(Login));
