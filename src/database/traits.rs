use failure::{self, Error, ResultExt};
use diesel;
use diesel::prelude::*;
use bcrypt;

use database::User;
use database::schema::users;

pub trait Database {
    fn authenticate_user(&self, username: &str, password: &str) -> Result<User, Error>;
    fn create_user(&self, username: &str, password: &str, is_admin: bool) -> Result<User, Error>;
}

impl Database for PgConnection {
    fn authenticate_user(&self, username: &str, password: &str) -> Result<User, Error> {
        let user: User = users::table
            .filter(users::username.eq(&username))
            .first(self)
            .context("The user doesn't exist")?;

        let is_valid = bcrypt::verify(password, &user.password_hash)
            .context("Unable to verify passwords with bcrypt")?;

        if is_valid {
            Ok(user)
        } else {
            Err(failure::err_msg("Invalid password"))
        }
    }

    fn create_user(&self, username: &str, password: &str, is_admin: bool) -> Result<User, Error> {
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
            .execute(self)
            .context("Unable to create new user")?;

        let got: User = users::table
            .filter(users::username.eq(&username))
            .first(self)?;

        Ok(got)
    }
}
