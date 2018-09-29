import React, { Component } from "react";
import { startLogin } from "../reducers";
import store from "../store";

export default class Login extends Component {
  constructor(props) {
    super(props);

    this.state = {
      username: "",
      password: ""
    };

    this.handleChange = this.handleChange.bind(this);
    this.submit = this.submit.bind(this);
  }

  submit(e) {
    e.preventDefault();
    store.dispatch(startLogin("/api", this.state.username, this.state.password));
  }

  handleChange(e) {
    const change = {};
    change[e.target.name] = e.target.value;
    this.setState(change);
  }

  render() {
    const { username, password } = this.state;

    return (
      <div id="login-container">
        <h1>Log In</h1>
        <form method="post" onSubmit={this.submit}>
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

