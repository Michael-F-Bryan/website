import { BrowserRouter as Router, Route } from 'react-router-dom'
import React, { Component } from 'react'
import { connect } from 'react-redux'

import Home from './pages/Home'

import './App.css';

class App extends Component {
  render() {
    return (
      <div className="App">
        <header className="App-header">
          <h1 className="App-title">Welcome to React</h1>
          <Router>
            <Route path="/" component={ Home } />
          </Router>
        </header>
      </div>
    )
  }
}

function mapToProps(state) {
  return state
}
export default connect(mapToProps)(App)