use std::ops::Deref;
use std::process::Command;
use failure::{self, Error, ResultExt};
use uuid::Uuid;
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

use database::Postgres;

/// A pool of connections to a postgres database.
pub struct PostgresPool(pub Pool<ConnectionManager<PgConnection>>);

impl PostgresPool {
    /// Create a new `PostgresPool` using the provided database URL.
    pub fn new<S: Into<String>>(database_url: S) -> Result<PostgresPool, Error> {
        let manager = ConnectionManager::new(database_url);
        let pool = Pool::new(manager).context("Unable to connect to the database")?;

        Ok(PostgresPool(pool))
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
    pub fn temporary() -> Result<PostgresPool, Error> {
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

        PostgresPool::new(db_string)
    }

    /// Creates a temporary postgres database and initializes it with:
    ///
    /// - An admin user (admin/admin)
    /// - A normal user (michael/password)
    pub fn with_fixtures() -> Result<PostgresPool, Error> {
        let pool = PostgresPool::temporary()?;

        let conn = pool.new_connection()?;
        apply_fixtures(&conn)?;

        Ok(pool)
    }

    /// Get a new connection to the database.
    pub fn new_connection(&self) -> Result<Postgres, Error> {
        let conn = self.0.get()?;

        Ok(Postgres(conn))
    }
}

fn apply_fixtures(db: &Postgres) -> Result<(), Error> {
    db.create_user("admin", "admin", true)?;
    db.create_user("michael", "password", false)?;

    Ok(())
}

impl Deref for PostgresPool {
    type Target = Pool<ConnectionManager<PgConnection>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
