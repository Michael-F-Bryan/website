#![feature(plugin)]
#![feature(use_extern_macros)]
#![plugin(rocket_codegen)]

extern crate log;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use rocket::{Request, Rocket};
use rocket::response::NamedFile;
use rocket_contrib::Template;

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
