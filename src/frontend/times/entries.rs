//! Timesheet entry endpoints.

use rocket::Route;
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;

/// All timesheet entry endpoints, relative to `/times`.
pub fn routes() -> Vec<Route> {
    routes![entry_overview, new_entry]
}

#[get("/")]
pub fn entry_overview(user: LoggedInUser) -> Template {
    Template::render("times/entries", json!{{"username": user.as_ref()}})
}

#[get("/new")]
pub fn new_entry(user: LoggedInUser) -> Template {
    Template::render("times/new_entry", json!{{"username": user.as_ref()}})
}
