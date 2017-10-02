use bcrypt::{self, DEFAULT_COST};
use uuid::Uuid;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::common::WriteConcern;

use super::User;
use db::DbConn;
use errors::*;


/// Authentication and user management.
pub trait Auth {
    fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>>;
    fn new_user(&mut self, username: &str, password: &str, is_admin: bool) -> Result<User>;
    fn validate_user(&self, username: &str, password: &str) -> Result<Option<User>>;
}


impl Auth for DbConn {
    fn get_user_by_id(&self, uuid: Uuid) -> Result<Option<User>> {
        let filter = doc!{"uuid" => (uuid.to_string())};
        self.find_by("users", filter)
    }

    fn new_user(&mut self, username: &str, password: &str, is_admin: bool) -> Result<User> {
        info!(
            "Creating new {} {:?}",
            if is_admin { "admin" } else { "user" },
            username
        );

        // first make sure the user doesn't already exist
        if self.find_by::<User>("users", doc!{ "name" => username})?
            .is_some()
        {
            bail!("Someone with that username already exists");
        }

        let hash = bcrypt::hash(password, DEFAULT_COST)?;

        let user = User {
            uuid: Uuid::new_v4(),
            name: username.to_string(),
            password_hash: hash,
            admin: is_admin,
        };

        // make sure we block until the new user has been written to disk
        let write_concerns = WriteConcern {
            j: true,
            fsync: true,
            ..Default::default()
        };
        self.db("website")
            .collection("users")
            .insert_one(user.clone().into(), Some(write_concerns))?;

        debug!("User created");

        Ok(user)
    }

    fn validate_user(&self, username: &str, password: &str) -> Result<Option<User>> {
        debug!("Validating username and password for {:?}", username);

        let filter = doc!{"name" => username};
        let got = self.find_by::<User>("users", filter)?;

        match got {
            Some(user) => if bcrypt::verify(password, &user.password_hash)? {
                debug!("Username and password valid for {:?}", user.name);
                Ok(Some(user))
            } else {
                debug!("Incorrect password for {:?}", username);
                Ok(None)
            },
            None => {
                debug!("User not found: {:?}", username);
                Ok(None)
            }
        }
    }
}
