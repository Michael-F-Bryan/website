//! The various endpoints and utilities for the website server.

#[macro_use]
pub mod utils;
pub mod auth;
pub mod times;
pub mod root;
pub mod errors;

use rocket::Rocket;
use rocket_contrib::Template;

/// Create a web server with all endpoints set up and error handlers configured,
/// you just need to add the backing database as [Managed State].
///
/// [Managed State]: https://rocket.rs/guide/state/#managed-state
pub fn create_server() -> Rocket {
    let mut r = Rocket::ignite();

    r = errors::mount_errors(r);

    r = r.mount("/", root::endpoints())
        .mount("/", auth::endpoints())
        .mount("/times", times::endpoints());

    r.attach(Template::fairing())
}
