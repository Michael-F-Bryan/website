#![recursion_limit = "128"]
#![feature(plugin, custom_derive)]
#![feature(use_extern_macros)]
#![plugin(rocket_codegen)]

extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate failure;
#[macro_use(log)]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

#[cfg(test)]
extern crate diesel_migrations;

pub mod frontend;
pub mod database;

pub use frontend::create_server;
