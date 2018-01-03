//! Timesheet entry endpoints.

use chrono::{Local, NaiveDateTime};
use rocket::{Response, Route};
use rocket::response::Redirect;
use rocket::request::{Form, FromForm};
use rocket_contrib::Template;
use frontend::auth::LoggedInUser;
use frontend::utils::LoginRequired;

/// All timesheet entry endpoints, relative to `/times`.
pub fn routes() -> Vec<Route> {
    routes![new_entry, create_entry]
}

#[get("/new")]
pub fn new_entry(user: LoginRequired<LoggedInUser>) -> Template {
    let ctx = json!{{
        "title": "New Entry",
        "username": user.as_ref(),
        "default_date": Local::now().format("%Y-%m-%d").to_string(),
        "start_time": Local::now().format("%H:%M").to_string(),
        "end_time": "17:00",
        "breaks": 0,
        "morning": "",
        "afternoon": "",
    }};

    Template::render("times/new_entry", ctx)
}

#[derive(Debug, FromForm)]
pub struct NewEntry {
    pub date: String,
    pub start: String,
    pub end: String,
    pub breaks: usize,
    pub morning: String,
    pub afternoon: String,
}

#[post("/new", data = "<entry>")]
pub fn create_entry(
    user: LoginRequired<LoggedInUser>,
    entry: Form<NewEntry>,
) -> Result<Redirect, Template> {
    unimplemented!()
}
