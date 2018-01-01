//! Endpoints related to managing timesheet stuff.

pub mod entries;
pub mod slices;

use rocket::{Route};
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;
use frontend::utils::{self, LoginRequired};

pub fn endpoints() -> Vec<Route> {
    let mut routes = routes![overview];
    routes.extend(utils::prefix_routes("/entries", entries::routes()));
    routes.extend(utils::prefix_routes("/slice", slices::routes()));

    routes
}

/// Generate a general overview of the logged-in user's timesheets.
#[get("/")]
pub fn overview(user: LoginRequired<LoggedInUser>) -> Template {
    // TODO: Fetch all timesheet entries
    // TODO: Fetch all time slices
    let ctx = json!{{
    "username": user.as_ref(),
    "entries": [],
    "slices": [],
    }};

    Template::render("times/overview", ctx)
}
