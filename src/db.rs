use std::ops::{Deref, DerefMut};
use std::env;
use rocket::request::{self, FromRequest, Request, State};
use rocket::http::Status;
use rocket::Outcome;
use mongodb::{Client, ThreadedClient};

use errors::*;


pub fn connect() -> Result<Client> {
    let db_url = env::var("DATABASE_URL").unwrap();
    Client::with_uri(&db_url).chain_err(|| "Couldn't connect to the database")
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
