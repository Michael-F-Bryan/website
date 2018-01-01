use std::path::{Path, PathBuf};
use rocket::Rocket;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;

pub fn mount_endpoints(r: Rocket) -> Rocket {
    r.mount("/", routes![home, home_authenticated, static_assets])
}

/// Serves up the static assets under `/static/`.
#[get("/static/<file..>")]
pub fn static_assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// The homepage.
#[get("/", rank = 0)]
pub fn home_authenticated(user: LoggedInUser) -> Template {
    Template::render("home", json!{{"username": user.as_ref()}})
}

/// The homepage.
#[get("/", rank = 2)]
pub fn home() -> Template {
    Template::render("home", json!{{"username": null}})
}
