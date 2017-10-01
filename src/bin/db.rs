use std::ops::Deref;
use std::env;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;
use rocket::request::{self, FromRequest, Request, State};
use rocket::http::Status;
use rocket::Outcome;


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
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
