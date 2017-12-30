use std::path::{Path, PathBuf};
use std::ops::Deref;

use rocket::{self, Outcome, Request, Rocket, State};
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::request::{self, FromRequest};
use rocket_contrib::Template;
use database::Database;

/// Create a web server with all endpoints set up and error handlers configured,
/// you just need to add the backing database as [Managed State].
///
/// [Managed State]: https://rocket.rs/guide/state/#managed-state
pub fn create_server() -> Rocket {
    Rocket::ignite()
        .catch(errors![not_found])
        .mount("/", routes![home])
        .attach(Template::fairing())
}

/// The 404 handler.
#[error(404)]
pub fn not_found(_: &Request) -> Template {
    let context = json!{{}};
    Template::render("not_found", context)
}

/// Serves up the static assets under `/static/`.
#[get("/static/<file..>")]
pub fn static_assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// The homepage.
#[get("/")]
pub fn home() -> Template {
    let context = json!{{}};
    Template::render("home", context)
}

/// The interface used
pub trait DatabasePool {
    fn database(&self) -> Box<Database>;
}

/// A connection guard providing access to the database.
pub struct DbConn(Box<Database>);

impl Deref for DbConn {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(_request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        unimplemented!()
        // let pool = request.guard::<State<Pool>>()?;
        // Outcome::Success(DbConn(pool.database()))
    }
}
