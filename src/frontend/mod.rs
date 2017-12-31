//! The various endpoints and utilities for the website server.

pub mod auth;

use std::path::{Path, PathBuf};

use rocket::{self, Request, Rocket};
use rocket::response::NamedFile;
use rocket_contrib::{Template, Value};
use self::auth::LoggedInUser;

/// Create a web server with all endpoints set up and error handlers configured,
/// you just need to add the backing database as [Managed State].
///
/// [Managed State]: https://rocket.rs/guide/state/#managed-state
pub fn create_server() -> Rocket {
    Rocket::ignite()
        .catch(errors![not_found])
        .mount("/", routes![home, static_assets])
        .mount("/", auth::routes())
        .attach(Template::fairing())
}

/// The 404 handler.
#[error(404)]
pub fn not_found(_: &Request) -> Template {
    let context = json!{{"username": null}};
    Template::render("not_found", context)
}

/// Serves up the static assets under `/static/`.
#[get("/static/<file..>")]
pub fn static_assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// The homepage.
#[get("/")]
pub fn home(user: Option<LoggedInUser>) -> Template {
    let ctx = base_context(user);
    Template::render("home", ctx)
}

fn base_context<L>(user: Option<L>) -> Value
where
    L: AsRef<str>,
{
    json!{{
        "username": user.map(|u| u.as_ref().to_string())
    }}
}
