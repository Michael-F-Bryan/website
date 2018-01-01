//! Root endpoints like the home page.

use std::path::{Path, PathBuf};
use rocket::Rocket;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;
use frontend::utils::Cached;

pub fn mount_endpoints(r: Rocket) -> Rocket {
    r.mount(
        "/",
        routes![
            home_authenticated,
            home,
            static_assets,
            resume,
            resume_authenticated,
            favicon,
        ],
    )
}

/// Serves up the static assets under `/static/`.
#[get("/static/<file..>")]
pub fn static_assets(file: PathBuf) -> Option<Cached<NamedFile>> {
    NamedFile::open(Path::new("static/").join(file))
        .map(Cached::from)
        .ok()
}

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::moved("/static/favicon.ico")
}
/// The homepage.
#[get("/")]
pub fn home_authenticated(user: LoggedInUser) -> Cached<Template> {
    Template::render("home", json!{{"username": user.as_ref()}}).into()
}

/// The homepage.
#[get("/", rank = 2)]
pub fn home() -> Cached<Template> {
    Template::render("home", json!{{"username": null}}).into()
}

#[get("/resume")]
pub fn resume_authenticated(user: LoggedInUser) -> Cached<Template> {
    Template::render("resume", json!{{"username": user.as_ref()}}).into()
}

#[get("/resume", rank = 2)]
pub fn resume() -> Cached<Template> {
    Template::render("resume", json!{{"username": null}}).into()
}
