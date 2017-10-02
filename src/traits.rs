#![allow(unused_variables, unused_imports)]

use std::io::Write;
use std::convert::TryFrom;
use bcrypt::{self, DEFAULT_COST};
use uuid::Uuid;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use serde_json;
use bson::Document;

use models::User;
use db::DbConn;
use errors::*;


pub trait Auth {
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

/// An in-memory representation of the entire contents of the database.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatabaseContents {
    pub users: Vec<User>,
}

impl DataStore for DbConn {
    fn dump_database(&self, writer: &mut Write) -> Result<()> {
        let users = self.find_many("users", doc!{})?
            .collect::<Result<Vec<User>>>()?;

        let db_contents = DatabaseContents { users };
        serde_json::to_writer_pretty(writer, &db_contents)?;

        Ok(())
    }

    fn load_database(&mut self, data: &[u8]) -> Result<()> {
        let got: DatabaseContents = serde_json::from_slice(data)?;
        let DatabaseContents { users } = got;

        let users: Vec<Document> = users.into_iter().map(Into::into).collect();
        self.collection("users").insert_many(users, None)?;

        Ok(())
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
