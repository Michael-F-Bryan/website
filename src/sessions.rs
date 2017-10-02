use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rocket::request::{self, FromRequest, Request, State};
use rocket::http::{Cookie, Status};
use rocket::Outcome;
use uuid::Uuid;
use models::User;
use errors::*;

use db::DbConn;


pub const AUTH_COOKIE: &'static str = "website-session";

/// An object which allows you to keep track of login sessions.
#[derive(Debug, Clone)]
pub struct SessionManager(Arc<RwLock<HashMap<Uuid, Session>>>);

impl SessionManager {
    pub fn new() -> SessionManager {
        Default::default()
    }

    pub fn get(&self, uuid: &Uuid) -> Option<Session> {
        self.0.read().unwrap().get(uuid).cloned()
    }

    /// Create a new session for a user.
    pub fn new_session(&self, user: &User) -> Session {
        let session_id = Uuid::new_v4();
        let session = Session {
            session_id: session_id,
            user_id: user.uuid,
            username: user.name.clone(),
        };

        self.0.write().unwrap().insert(session_id, session.clone());
        session
    }
}

impl Default for SessionManager {
    fn default() -> SessionManager {
        let session = HashMap::new();
        SessionManager(Arc::new(RwLock::new(session)))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for SessionManager {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let session_manager = request.guard::<State<SessionManager>>().map_failure(|_| {
            (
                Status::InternalServerError,
                "Couldn't get the session manager".into(),
            )
        })?;

        Outcome::Success(session_manager.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
}

impl Session {
    pub fn cookie(&self) -> Cookie<'static> {
        Cookie::new(String::from(AUTH_COOKIE), self.session_id.to_string())
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = <DbConn as FromRequest<'a, 'r>>::Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let session_manager = SessionManager::from_request(request)?;

        let cookies = request.cookies();
        let auth_cookie = match cookies.get(AUTH_COOKIE) {
            Some(cookie) => cookie,
            None => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    "No authentication cookie found".into(),
                ))
            }
        };

        let id: Uuid = match auth_cookie.value().parse() {
            Ok(v) => v,
            Err(_) => {
                return Outcome::Failure((
                    Status::BadRequest,
                    "Couldn't parse authentication cookie".into(),
                ))
            }
        };

        match session_manager.get(&id) {
            Some(sess) => Outcome::Success(sess),
            None => Outcome::Failure((Status::Unauthorized, "Login session is not valid".into())),
        }
    }
}
