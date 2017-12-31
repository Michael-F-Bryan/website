#![feature(plugin)]
#![feature(use_extern_macros)]
#![plugin(rocket_codegen)]

extern crate bcrypt;
#[macro_use]
extern crate diesel;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate log;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[cfg(test)]
extern crate diesel_migrations;

pub mod frontend;
pub mod database;

pub use frontend::create_server;
