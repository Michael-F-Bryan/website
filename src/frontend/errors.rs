//! The website's error handlers.

use rocket::{self, Request, Rocket};
use rocket_contrib::Template;

pub fn mount_errors(r: Rocket) -> Rocket {
    r.catch(errors![not_found, unauthorized])
}

#[error(404)]
pub fn not_found(_: &Request) -> Template {
    let context = json!{{"username": null}};
    Template::render("not_found", context)
}

#[error(401)]
pub fn unauthorized(_: &Request) -> Template {
    let context = json!{{"username": null}};
    Template::render("unauthorized", context)
}
