use std::ops::Deref;
use std::env;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;
use rocket::request::{self, FromRequest, Request, State};
use rocket::http::Status;
use rocket::Outcome;
use website::models::User;
use website::traits::DataStore;
use website::errors::*;

pub const AUTH_COOKIE: &'static str = "website-session";


// An alias to the type for a pool of Diesel SQLite connections.
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Initializes a database pool.
pub fn init_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").unwrap();
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>().map_failure(|_| {
            (
                Status::InternalServerError,
                "Couldn't get `Pool` request guard".into(),
            )
        })?;


        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((
                Status::ServiceUnavailable,
                Error::from("Couldn't connect to the database"),
            )),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoggedInUser(pub User);

impl<'a, 'r> FromRequest<'a, 'r> for LoggedInUser {
    type Error = <DbConn as FromRequest<'a, 'r>>::Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let db = DbConn::from_request(request)?;
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

        let id: usize = match auth_cookie.value().parse() {
            Ok(v) => v,
            Err(_) => {
                return Outcome::Failure((
                    Status::BadRequest,
                    "Couldn't parse authentication cookie".into(),
                ))
            }
        };

        match db.get_user_by_id(id) {
            Some(user) => Outcome::Success(LoggedInUser(user)),
            None => return Outcome::Failure((Status::BadRequest, "User doesn't exist".into())),
        }
    }
}

impl Deref for LoggedInUser {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
