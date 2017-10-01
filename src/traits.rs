#![allow(unused_variables, unused_imports)]

use std::io::Write;
use std::convert::TryFrom;
use bcrypt::{self, DEFAULT_COST};
use uuid::Uuid;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use models::User;
use db::DbConn;
use errors::*;


pub trait Auth: DataStore {
    fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>>;
    fn new_user(&mut self, username: &str, password: &str, is_admin: bool) -> Result<User>;
    fn validate_user(&self, username: &str, password: &str) -> Result<Option<User>>;
}

pub trait DataStore {
    /// Write a serialized version of the entire database to some `Writer`.
    fn dump_database(&self, writer: &mut Write) -> Result<()>;
    /// Load a serialized version of the database.
    fn load_database(&mut self, data: &[u8]) -> Result<()>;
}


impl DataStore for DbConn {
    fn dump_database(&self, writer: &mut Write) -> Result<()> {
        unimplemented!()
    }

    fn load_database(&mut self, data: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

impl Auth for DbConn {
    fn get_user_by_id(&self, uuid: Uuid) -> Result<Option<User>> {
        let filter = doc!{ "uuid" => (uuid.to_string())};
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

        self.db("website")
            .collection("users")
            .insert_one(user.clone().into(), None)?;

        debug!("User created");

        Ok(user)
    }

    fn validate_user(&self, username: &str, password: &str) -> Result<Option<User>> {
        debug!("Validating username and password for {:?}", username);
        let hash = bcrypt::hash(password, DEFAULT_COST)?;

        let filter = doc!{"name" => (username), "password_hash" => (hash)};
        self.find_by("users", filter)
    }
}
