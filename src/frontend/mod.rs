use std::path::{Path, PathBuf};
use std::ops::Deref;

use rocket::{self, Request, Rocket};
use rocket::response::NamedFile;
use rocket_contrib::Template;
use database::Database;

/// Create the web server.
pub fn create_server() -> Rocket {
    Rocket::ignite()
        .catch(errors![not_found])
        .mount("/", routes![home])
        .attach(Template::fairing())
}

#[error(404)]
fn not_found(_: &Request) -> Template {
    let context = json!{{}};
    Template::render("not_found", context)
}

#[get("/static/<file..>")]
fn static_assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/")]
fn home() -> Template {
    let context = json!{{}};
    Template::render("home", context)
}

pub trait DatabasePool {
    fn database(&self) -> Box<Database>;
}

pub struct DbConn(Box<Database>);

impl Deref for DbConn {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
