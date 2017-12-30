#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::Rocket;

/// Create the web server.
pub fn create_server() -> Rocket {
    Rocket::ignite()
}
