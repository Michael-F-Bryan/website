//! The various endpoints and utilities for the website server.

pub mod auth;
pub mod times;
pub mod misc;

use rocket::{self, Request, Rocket};
use rocket_contrib::Template;

/// Create a web server with all endpoints set up and error handlers configured,
/// you just need to add the backing database as [Managed State].
///
/// [Managed State]: https://rocket.rs/guide/state/#managed-state
pub fn create_server() -> Rocket {
    let mut r = Rocket::ignite().catch(errors![not_found]);

    r = misc::mount_endpoints(r);
    r = auth::mount_endpoints(r);
    r = times::mount_endpoints(r);

    r.attach(Template::fairing())
}

/// The 404 handler.
#[error(404)]
pub fn not_found(_: &Request) -> Template {
    let context = json!{{"username": null}};
    Template::render("not_found", context)
}
