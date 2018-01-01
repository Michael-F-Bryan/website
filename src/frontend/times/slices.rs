//! Time slice endpoints.

use rocket::Route;
use rocket_contrib::Template;
use frontend::auth::{LoggedInUser, LoginRequired};

/// All timesheet entry endpoints, relative to `/times`.
pub fn routes() -> Vec<Route> {
    routes![new_slice]
}

#[get("/new")]
pub fn new_slice(user: LoginRequired<LoggedInUser>) -> Template {
    Template::render("times/new_slice", json!{{"username": user.as_ref()}})
}
