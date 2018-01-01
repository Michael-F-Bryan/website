//! Timesheet entry endpoints.

use rocket::Route;
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;
use frontend::utils::LoginRequired;

/// All timesheet entry endpoints, relative to `/times`.
pub fn routes() -> Vec<Route> {
    routes![new_entry]
}

#[get("/new")]
pub fn new_entry(user: LoginRequired<LoggedInUser>) -> Template {
    Template::render("times/new_entry", json!{{"username": user.as_ref()}})
}
