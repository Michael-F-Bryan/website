//! The website's underlying data model.
//!
//! # Examples
//!
//! User management is usually fairly straightforward, once you've established a
//! database connection you just use the corresponding methods on [`Postgres`].
//!
//! ```rust,no_run
//! # extern crate failure;
//! # extern crate website;
//! use website::database::{Postgres, PostgresPool};
//!
//! # fn run() -> Result<(), failure::Error> {
//! // Connect to the database and grab a connection
//! let db = PostgresPool::new("DATABASE_URL")?;
//! let conn = db.new_connection()?;
//!
//! // create a new user
//! conn.create_user("michael", "Password1", true)?;
//!
//! // then make sure he can log in
//! assert!(conn.authenticate("michael", "Password1").is_ok());
//!
//! // and finally we'll print out all users in the database
//! let users = conn.list_users()?;
//! println!("{:#?}", users);
//!
//! # Ok(())
//! # }
//! # fn main() {}
//! ```
//!
//! [`Postgres`]: struct.Postgres.html

#![deny(missing_docs)]

/// Automatically generated database bindings used with [Diesel].
///
/// This is produced using the Diesel CLI tool:
///
/// ```bash
/// $ diesel print-schema > src/database/schema.rs
/// ```
///
/// [Diesel]: https://diesel.rs/
pub mod schema;
mod models;
mod utils;

pub use self::models::{TimesheetEntry, User};

pub use self::utils::PostgresPool;

use bcrypt;
use diesel;
use diesel::prelude::*;
use failure::{self, Error, ResultExt};
use log;
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use self::schema::users;

/// An interface to the underlying Postgres database.
///
/// For convenience, you can use use `Postgres` as a [Request Guard] with the
/// Rocket web framework. Simply add it as a parameter to your endpoint function
/// and you'll gain access to the database.
///
/// Refer to the [module-level documentation](index.html) for information on how
/// to use the `Postgres` connection.
///
/// [Request Guard]: https://rocket.rs/guide/requests/#request-guards
pub struct Postgres(PooledConnection<ConnectionManager<PgConnection>>);

impl Postgres {
    /// Fetch the number of users.
    pub fn num_users(&self) -> Result<usize, Error> {
        let got: i64 = users::table.count().get_result(&*self.0)?;
        Ok(got as usize)
    }

    /// Attempt to authenticate a user, returning its associated `User` struct
    /// (minus the password hash) if successful.
    pub fn authenticate_user(&self, username: &str, password: &str) -> Result<User, Error> {
        let mut user: User = users::table
            .filter(users::username.eq(&username))
            .first(&*self.0)
            .context("The user doesn't exist")?;

        let is_valid = bcrypt::verify(password, &user.password_hash)
            .context("Unable to verify passwords with bcrypt")?;

        if is_valid {
            // manually erase the password hash
            user.password_hash.clear();
            Ok(user)
        } else {
            Err(failure::err_msg("Invalid password"))
        }
    }

    /// Create a new user.
    pub fn create_user(
        &self,
        username: &str,
        password: &str,
        is_admin: bool,
    ) -> Result<User, Error> {
        #[derive(Insertable)]
        #[table_name = "users"]
        struct NewUser<'a> {
            username: &'a str,
            password_hash: &'a str,
            is_admin: bool,
        }

        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;

        let request = NewUser {
            username: username,
            password_hash: &password_hash,
            is_admin: is_admin,
        };

        diesel::insert_into(users::table)
            .values(&request)
            .execute(&*self.0)
            .context("Unable to create new user")?;

        let mut got: User = users::table
            .filter(users::username.eq(&username))
            .first(&*self.0)?;

        got.password_hash.clear();
        Ok(got)
    }

    /// Generate a list of all the current users.
    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        let mut got: Vec<User> = users::table
            .load(&*self.0)
            .context("Unable to fetch user list")?;

        for user in &mut got {
            user.password_hash.clear();
        }

        Ok(got)
    }

    /// The underlying database connection.
    pub fn inner(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Postgres {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Postgres, ()> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.new_connection() {
            Ok(conn) => Outcome::Success(conn),
            Err(e) => {
                log::warn!("Error fetching database connection: {}", e);
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}
