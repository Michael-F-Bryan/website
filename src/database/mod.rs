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
pub struct Postgres(PooledConnection<ConnectionManager<PgConnection>>);

impl Postgres {
    pub fn num_users(&self) -> Result<usize, Error> {
        let got: i64 = users::table.count().get_result(&*self.0)?;
        Ok(got as usize)
    }

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

        let got: User = users::table
            .filter(users::username.eq(&username))
            .first(&*self.0)?;

        Ok(got)
    }

    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        let mut got: Vec<User> = users::table
            .load(&*self.0)
            .context("Unable to fetch user list")?;

        for user in &mut got {
            user.password_hash.clear();
        }

        Ok(got)
    }

    pub fn inner(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Postgres {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Postgres, ()> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.connection() {
            Ok(conn) => Outcome::Success(conn),
            Err(e) => {
                log::warn!("Error fetching database connection: {}", e);
                Outcome::Failure((Status::InternalServerError, ()))
            }
        }
    }
}
