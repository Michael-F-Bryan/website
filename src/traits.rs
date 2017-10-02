use std::io::Write;
use bcrypt::{self, DEFAULT_COST};
use uuid::Uuid;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::common::WriteConcern;
use serde_json;
use bson::Document;

use models::User;
use times::{TimeSheetEntry, TIMESHEET_ENTRY_NAME};
use db::DbConn;
use errors::*;


/// Authentication and user management.
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
    pub timesheet_entries: Vec<TimeSheetEntry>,
}

impl DataStore for DbConn {
    fn dump_database(&self, writer: &mut Write) -> Result<()> {
        let users: Result<Vec<User>> = self.find_all("users")?.collect();
        let users = users?;
        let times: Result<Vec<TimeSheetEntry>> = self.find_all(TIMESHEET_ENTRY_NAME)?.collect();
        let timesheet_entries = times?;

        let db_contents = DatabaseContents {
            users,
            timesheet_entries,
        };
        serde_json::to_writer_pretty(writer, &db_contents)?;

        Ok(())
    }

    fn load_database(&mut self, data: &[u8]) -> Result<()> {
        let got: DatabaseContents = serde_json::from_slice(data)?;
        let DatabaseContents {
            users,
            timesheet_entries,
        } = got;

        let users: Vec<Document> = users.into_iter().map(Into::into).collect();
        self.collection("users").insert_many(users, None)?;

        let timesheet_entries: Vec<Document> =
            timesheet_entries.into_iter().map(Into::into).collect();
        self.collection(TIMESHEET_ENTRY_NAME)
            .insert_many(timesheet_entries, None)?;

        Ok(())
    }
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
