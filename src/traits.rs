use std::io::Write;
use diesel::connection::Connection;

use models::User;
use errors::*;


pub struct WebSite;

pub trait DataStore {
    fn get_user_by_id(&self, id: usize) -> Option<User>;

    /// Write a serialized version of the entire database to some `Writer`.
    fn dump_database(&self, writer: &mut Write) -> Result<usize>;
    /// Load a serialized version of the database.
    fn load_database(&self, data: &[u8]) -> Result<()>;
}


impl<C: Connection> DataStore for C {
    fn get_user_by_id(&self, id: usize) -> Option<User> {
        unimplemented!()
    }

    /// Write a serialized version of the entire database to some `Writer`.
    fn dump_database(&self, writer: &mut Write) -> Result<usize> {
        unimplemented!()
    }
    /// Load a serialized version of the database.
    fn load_database(&self, data: &[u8]) -> Result<()> {
        unimplemented!()
    }
}
