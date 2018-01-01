//! Endpoints related to managing timesheet stuff.

pub mod entries;
pub mod slices;

use rocket::Rocket;
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;
use frontend::utils::LoginRequired;

pub fn mount_endpoints(r: Rocket) -> Rocket {
    r.mount("/times", routes![overview])
        .mount("/times/entries", entries::routes())
        .mount("/times/slice", slices::routes())
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
