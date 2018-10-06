import { combineReducers } from 'redux'
import moment from "moment";

export const LOGIN_START = "LOGIN_START";
export const LOGIN_FAILED = "LOGIN_FAILED";
export const LOGIN_COMPLETE = "LOGIN_COMPLETE";
export const CLEAR_LOGIN_ERROR = "CLEAR_LOGIN_ERROR";
export const LOGOUT = "LOGOUT";
export const PING = "PING";

const InitialTimesState = [
  {
    id: 1, 
    start: new Date(2018, 9, 6, 8, 5), 
    end: new Date(2018, 9, 6, 17, 5), 
    breaks: 1000*60*30, 
    morning: "Did some stuff",
    afternoon: "Did some more stuff"
  },
  {
    id: 1, 
    start: new Date(2018, 8, 6, 8, 44), 
    end: new Date(2018, 8, 6, 17, 32), 
    breaks: 1000*60*30, 
    morning: "Did some stuff",
    afternoon: "Did some more stuff"
  }
];

const InitialLoginState = {
  login_state: "idle",
  username: null,
  api_root: "/api"
};

const website = combineReducers({
  login,
  times
});

export default website;

function login(state = InitialLoginState, action) {
  switch(action.type) {
    case LOGIN_START:
      return Object.assign({}, state, { login_state: "logging_in", username: action.username, error: null });

    case LOGIN_COMPLETE:
      return Object.assign({}, state, { login_state: "done", error: null });

    case LOGIN_FAILED:
      return Object.assign({}, state, { login_state: "error", error: action.error, username: null });

    case CLEAR_LOGIN_ERROR:
      return Object.assign({}, state, { login_state: "idle", error: null });

    case LOGOUT:
      return Object.assign({}, state, { login_state: "idle", username: null });

    case PING:
      const { username } = action.data;
      const newState = Object.assign({}, state, { username });
      if (username) {
        newState.login_state = "done";
      }
      return newState;

    default: 
      return state;
  }
}

function times(state = InitialTimesState, action) {
  switch(action.type) {
    default:
      return state;
  }
}

export function startLogin(api_root, username, password) {
  return function(dispatch) {
    dispatch({ type: LOGIN_START, username });

    return fetch(api_root + "/login", { 
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username, password 
      })
    })
      .then(
        response => response.json(),
        error => dispatch({ type: LOGIN_FAILED, error }),
      )
      .then(
        json => {
          if (json.error) {
            dispatch({ type: LOGIN_FAILED, error: json.error });
          } else {
            dispatch({ type: LOGIN_COMPLETE });
          }
        }, 
        error => dispatch({ type: LOGIN_FAILED, error }),
      );
  }
}

export function startLogout(api_root) {
  return function(dispatch) {
    return fetch(api_root + "/logout", { 
      method: "POST",
      headers: {
        "Accept": "application/json",
        "Content-Type": "application/json"
      }
    })
      .then(
        response => response.json(),
        error => console.log("Unable to logout:", error),
      )
      .then(
        json => {
          if (json.error) {
            console.log("Unable to logout:", json.error);
          } else {
            dispatch({ type: LOGOUT });
          }
        },
        error => console.log("Unable to logout:", error),
      );
  }
}

export function ping(api_root) {
  return function(dispatch) {
    return fetch(api_root + "/ping")
      .then(
        response => response.json(),
        error => console.log("Unable to send a ping")
      )
      .then(json => {
        dispatch({ type: PING, data: json });
      });
  };
}

export function saveTimesheetEntry(api_root, entry, success, error) {
  return function(dispatch) {
    var { id, date, start, end, breaks, morning, afternoon } = entry;

    date = moment(date, moment.HTML5_FMT.DATE);
    start = moment.duration(start, moment.HTML5_FMT.TIME);
    start = moment(date).add(start);
    end = moment.duration(end, moment.HTML5_FMT.TIME);
    end = moment(date).add(end);

    if (end.isAfter(start)) {
      error(new Error("You can't end before you've started"));
      return;
    }
    if (breaks < 0) {
      error(new Error("You can't have negative breaks"));
      return;
    }

    const endpoint = id ? api_root + "/timesheets/" + id : api_root + "/timesheets/new";
    const body = { start, end, breaks, morning, afternoon };

    return fetch(endpoint, {
      method: "POST",
      headers: {
        "Accept": "application/json",
        "Content-Type": "application/json"
      },
      body
    })
      .then(
        response => response.json(),
        err => error(err)
      )
      .then(
        json => json.success ? success(json) : error(json),
        err => error(err)
      );
  };
}

