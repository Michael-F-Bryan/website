import moment from "moment";
import Entry from "./Entry";

export const LOGIN_COMPLETE = "LOGIN_COMPLETE";
export const LOGOUT = "LOGOUT";
export const PING = "PING";
export const ENTRY_DELETE = "ENTRY_DELETE";
export const ENTRY_SAVED = "ENTRY_SAVED";
export const ENTRIES_UPDATED = "ENTRIES_UPDATED";

const InitialState = {
  username: "",
  api_root: "/api",
  times: [
    new Entry(0, new Date(2018, 9, 6, 8, 5), new Date(2018, 9, 6, 17, 5), 1000*60*30, "Drank **loads** of coffee", "Wrote Code"),
    new Entry(1, new Date(2018, 8, 6, 8, 44), new Date(2018, 8, 6, 17, 32), 1000*60*35, "Did some stuff", "Knocked off early"),
  ]
};

export default function login(state = InitialState, action) {
  switch(action.type) {
    case LOGIN_COMPLETE:
      return Object.assign({}, state, { username: action.username });

    case LOGOUT:
      return Object.assign({}, state, { username: "" });

    case PING:
      const { username } = action.data;
      return Object.assign({}, state, { username });

    case ENTRY_SAVED: {
      const { entry } = action;
      console.log(state.times);
      const times = state.times.map(e => e.clone());
      const ix = times.findIndex(e => e.id === entry.id);
      if (ix === -1) {
        times.push(entry);
      } else {
        times[ix] = entry;
      }
      return Object.assign({}, state, { times });
    }

    case ENTRY_DELETE: {
      const { id } = action;
      const times = state.filter(entry => entry.id !== id);
      return Object.assign({}, state, { times });
    }

    case ENTRIES_UPDATED: {
      const timesById = {};
      for(const entry of state.times) {
        timesById[entry.id] = entry.clone();
      }
      for (const entry of action.entries) {
        timesById[entry.id] = entry;
      }
      const times = Object.values(timesById);
      times.sort((left, right) => moment.utc(left.timeStamp).diff(moment.utc(right.timeStamp)));
      return Object.assign({}, state, { times });
    }

    default: 
      return state;
  }
}

export function startLogin(username, password) {
  return function(dispatch, getStore) {
    const { api_root } = getStore();

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
      )
      .then(json => {
          if (json.error) {
            throw new Error(json.error);
          } else {
            dispatch({ type: LOGIN_COMPLETE, username });
          }
        });
  }
}

export function startLogout() {
  return function(dispatch, getStore) {
    const { api_root } = getStore();

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

export function ping() {
  return function(dispatch, getStore) {
    const { api_root } = getStore();

    return fetch(api_root + "/ping")
      .then(response => response.json(), error => console.log("Unable to send a ping"))
      .then(json => dispatch({ type: PING, data: json }))
      .catch(error => console.error("Error encountered while sending a ping", error));
  };
}

export function saveTimesheetEntry(entry) {
  return function(dispatch, getStore) {
    const { api_root } = getStore();

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
      body: entry.toJSON()
    })
      .then(response => response.json())
      .then(json => {
          if (json.success) {
            const got = Entry.fromJSON(json.entry);
            dispatch({ type: ENTRY_SAVED, entry: got });
            return got;
          } else {
            throw new Error(json.error);
          }
        });
  };
}

export function deleteTimesheetEntry(id) {
  return function(dispatch, getStore) {
    const { api_root } = getStore();

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

export function fetchTimesheetEntries(start, end) {
  return function(dispatch, getStore) {
    const { api_root } = getStore();

    return fetch(api_root + "/timesheets", {
      method: "POST",
      headers: {
        "Accept": "application/json",
        "Content-Type": "application/json"
      },
      body: JSON.stringify({start, end})
    })
      .then(response => response.json())
      .then(json => {
          if (json.success) {
            dispatch({ type: ENTRIES_UPDATED, entries: json.entries.map(Entry.fromJSON) });
          }  else {
            throw new Error(json.error);
          }
        }
      )
      .catch(error => console.error("Unable to fetch entries", error));
  };
}
