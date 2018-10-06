import { combineReducers } from 'redux'

export const LOGIN_START = "LOGIN_START";
export const LOGIN_FAILED = "LOGIN_FAILED";
export const LOGIN_COMPLETE = "LOGIN_COMPLETE";
export const CLEAR_LOGIN_ERROR = "CLEAR_LOGIN_ERROR";
export const LOGOUT = "LOGOUT";

const InitialLoginState = {
  login_state: "idle",
  username: null,
  api_root: "/api"
};

const website = combineReducers({
  login
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

