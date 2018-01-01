pub mod entries;

use rocket::Rocket;
use rocket_contrib::Template;
use frontend::auth::{LoggedInUser, LoginRequired};

pub fn mount_endpoints(r: Rocket) -> Rocket {
    r.mount("/times", routes![overview])
        .mount("/times/entries", entries::routes())
}

#[get("/")]
pub fn overview(user: LoginRequired<LoggedInUser>) -> Template {
    Template::render("times/overview", json!{{"username": user.as_ref()}})
}
