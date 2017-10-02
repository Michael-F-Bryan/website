use std::ops::{Deref, DerefMut};
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use rocket::request::{self, FromRequest, Request, State};
use rocket::http::Status;
use rocket::Outcome;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use bson::Document;

use errors::*;


pub fn connect<S: AsRef<str>>(db_url: S) -> Result<Client> {
    Client::with_uri(db_url.as_ref()).chain_err(|| "Couldn't connect to the database")
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
    pub fn collection<S: AsRef<str>>(&self, name: S) -> Collection {
        self.db("website").collection(name.as_ref())
    }

    pub fn find_by<T>(&self, name: &str, filter: Document) -> Result<Option<T>>
    where
        T: TryFrom<Document, Error = Error> + Debug,
    {
        trace!(r#"Searching "{}" with filter {:?}"#, name, filter);

        let doc = self.collection(name).find_one(Some(filter), None)?;

        match doc {
            Some(doc) => {
                let got = doc.try_into()?;
                trace!("Found {:?}", got);
                Ok(Some(got))
            }
            None => {
                trace!("Didn't find anything");
                Ok(None)
            }
        }
    }

    pub fn _find_many<T>(
        &self,
        name: &str,
        filter: Option<Document>,
    ) -> Result<Box<Iterator<Item = Result<T>>>>
    where
        T: TryFrom<Document, Error = Error> + Debug + 'static,
    {
        trace!("Finding many from {:?} with filter {:?}", name, filter);

        let cursor = self.collection(name).find(filter, None)?;
        let items = cursor.map(|item| {
            item.map_err(|e| e.into()).and_then(TryInto::try_into)
        });

        Ok(Box::new(items))
    }

    pub fn find_all<T>(&self, collection: &str) -> Result<Box<Iterator<Item = Result<T>>>>
    where
        T: TryFrom<Document, Error = Error> + Debug + 'static,
    {
        self._find_many(collection, None)
    }

    pub fn find_many<T>(
        &self,
        collection: &str,
        filter: Document,
    ) -> Result<Box<Iterator<Item = Result<T>>>>
    where
        T: TryFrom<Document, Error = Error> + Debug + 'static,
    {
        self._find_many(collection, Some(filter))
    }
}
