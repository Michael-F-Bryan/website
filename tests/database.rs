extern crate diesel;
extern crate diesel_migrations;
extern crate failure;
extern crate uuid;
extern crate website;

use std::ops::Deref;
use std::process::Command;
use failure::{Error, ResultExt};
use uuid::Uuid;
use diesel::prelude::*;
use website::database::schema::users::dsl::*;
use website::database::Database;

#[test]
fn create_a_user() {
    let db = Postgres::new().unwrap();

    assert_eq!(db.num_users().unwrap(), 0);

    let got = db.create_user("michael", "password", true).unwrap();

    assert_eq!(got.username, "michael");
    assert!(got.is_admin);

    assert_eq!(db.num_users().unwrap(), 1);
}

#[test]
fn authenticate_user() {
    let db = Postgres::new().unwrap();
    db.create_user("michael", "password", true).unwrap();

    assert!(db.authenticate_user("michael", "some invalid pw").is_err());
    assert!(db.authenticate_user("michael", "password").is_ok());
}

/// Create a "temporary" postgres database we can connect to.
///
/// # Note
///
/// This *doesn't* drop the temporary database afterwards. The best way to deal
/// with this is by using a docker container which can be blown away afterwards.
///
/// ```bash
/// $ docker run --rm --detach -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgres
/// ```
struct Postgres {
    conn: PgConnection,
}

impl Postgres {
    pub fn new() -> Result<Postgres, Error> {
        let db_name = format!("database-{}", Uuid::new_v4().hyphenated()).replace("-", "_");
        let db_string = format!("postgres://postgres:postgres@localhost/{}", db_name);

        let output = Command::new("diesel")
            .arg("database")
            .arg("setup")
            .env("DATABASE_URL", &db_string)
            .output()
            .context("Unable to invoke `diesel`, is it installed?")?;

        if !output.status.success() {
            eprintln!("{:?}", output);
            return Err(failure::err_msg("Initializing database failed"));
        }

        println!("Database: {}", db_string);
        let conn = PgConnection::establish(&db_string)
            .context("Couldn't connect to the database. Is docker running?")?;

        Ok(Postgres { conn })
    }

    fn num_users(&self) -> Result<usize, Error> {
        let got: i64 = users.count().get_result(&self.conn)?;
        Ok(got as usize)
    }
}

impl Deref for Postgres {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}
