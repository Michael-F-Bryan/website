pub mod entries;
pub mod slices;

use rocket::Rocket;
use rocket_contrib::Template;
use frontend::auth::{LoggedInUser, LoginRequired};

pub fn mount_endpoints(r: Rocket) -> Rocket {
    r.mount("/times", routes![overview])
        .mount("/times/entries", entries::routes())
        .mount("/times/slice", slices::routes())
}

#[get("/")]
pub fn overview(user: LoginRequired<LoggedInUser>) -> Template {
    // TODO: Fetch all timesheet entries
    // TODO: Fetch all time slices
    Template::render("times/overview", json!{{"username": user.as_ref()}})
}
