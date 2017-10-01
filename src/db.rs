use std::ops::{Deref, DerefMut};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::env;
use rocket::request::{self, FromRequest, Request, State};
use rocket::http::Status;
use rocket::Outcome;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson::Document;

use errors::*;


pub fn connect(db_url: &str) -> Result<Client> {
    Client::with_uri(db_url).chain_err(|| "Couldn't connect to the database")
}


pub struct DbConn(pub Client);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let base_client = request.guard::<State<Client>>().map_failure(|_| {
            (
                Status::InternalServerError,
                "Couldn't get `Pool` request guard".into(),
            )
        })?;

        Outcome::Success(DbConn(base_client.clone()))
    }
}

impl Deref for DbConn {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DbConn {
    pub fn find_by<T>(&self, collection: &str, filter: Document) -> Result<Option<T>>
    where
        T: TryFrom<Document, Error = Error> + Debug,
    {
        debug!(r#"Searching "{}" with filter {:?}"#, collection, filter);

        let doc = self.db("website")
            .collection(collection)
            .find_one(Some(filter), None)?;

        match doc {
            Some(doc) => {
                let got = doc.try_into()?;
                debug!("Found {:?}", got);
                Ok(Some(got))
            }
            None => {
                debug!("Didn't find anything");
                Ok(None)
            }
        }
    }
}
