#![feature(plugin)]
#![feature(use_extern_macros)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate log;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod frontend;
pub mod database;

pub use frontend::create_server;
