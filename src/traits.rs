#![allow(unused_variables, unused_imports)]

use std::io::Write;
use bcrypt::{self, DEFAULT_COST};
use uuid::Uuid;
use mongodb::Client;

use models::User;
use errors::*;


pub trait Auth {
    fn get_user_by_id(&self, user_id: Uuid) -> Option<User>;
    fn new_user(&mut self, username: &str, password: &str, is_admin: bool) -> Result<User>;
    fn validate_user(&self, username: &str, password: &str) -> Option<User>;
}

pub trait DataStore {
    /// Write a serialized version of the entire database to some `Writer`.
    fn dump_database(&self, writer: &mut Write) -> Result<()>;
    /// Load a serialized version of the database.
    fn load_database(&mut self, data: &[u8]) -> Result<()>;
}


impl DataStore for Client {
    fn dump_database(&self, writer: &mut Write) -> Result<()> {
        unimplemented!()
    }

    fn load_database(&mut self, data: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

impl Auth for Client {
    fn get_user_by_id(&self, uuid: Uuid) -> Option<User> {
        unimplemented!()
    }

    fn new_user(&mut self, username: &str, password: &str, is_admin: bool) -> Result<User> {
        unimplemented!()
    }

    fn validate_user(&self, username: &str, password: &str) -> Option<User> {
        unimplemented!()
    }
}
