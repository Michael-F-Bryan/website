import { combineReducers } from 'redux'
import Entry from "./Entry";

export const LOGIN_START = "LOGIN_START";
export const LOGIN_FAILED = "LOGIN_FAILED";
export const LOGIN_COMPLETE = "LOGIN_COMPLETE";
export const CLEAR_LOGIN_ERROR = "CLEAR_LOGIN_ERROR";
export const LOGOUT = "LOGOUT";
export const PING = "PING";
export const ENTRY_DELETE = "ENTRY_DELETE";
export const ENTRY_SAVED = "ENTRY_SAVED";

const InitialTimesState = [
  new Entry(0, new Date(2018, 9, 6, 8, 5), new Date(2018, 9, 6, 17, 5), 1000*60*30, "Drank **loads** of coffee", "Wrote Code"),
  new Entry(1, new Date(2018, 8, 6, 8, 44), new Date(2018, 8, 6, 17, 32), 1000*60*35, "Did some stuff", "Knocked off early"),
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
    case ENTRY_SAVED:
      throw new Error("Not Implemented");

    case ENTRY_DELETE:
      const { id } = action;
      return state.filter(entry => entry.id !== id);

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

export function saveTimesheetEntry(api_root, entry) {
  return function(dispatch) {
    try {
      entry.validate();
    } catch (e) {
      return Promise.reject(e);
    }

    const { id } = entry;
    const endpoint = id ? api_root + "/timesheets/" + id : api_root + "/timesheets/new";

    return fetch(endpoint, {
      method: "POST",
      headers: {
        "Accept": "application/json",
        "Content-Type": "application/json"
      },
      body: entry
    })
      .then(response => response.json())
      .then(json => {
          if (json.success) {
            dispatch({ type: ENTRY_SAVED, json });
          } else {
            throw new Error(json.error);
          }
        });
  };
}

export function deleteTimesheetEntry(api_root, id) {
  return function(dispatch) {
    return fetch(api_root + "/timesheets/" + id, {
      method: "DELETE",
      headers: {
        "Accept": "application/json",
        "Content-Type": "application/json"
      }
    })
      .then(response => response.json())
      .then(json => {
          if (json.success) {
            dispatch({ type: ENTRY_DELETE, id });
          }  else {
            throw new Error(json.error);
          }
        }
      );
  }
}
