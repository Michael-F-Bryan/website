#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate bcrypt;
extern crate dotenv;
#[macro_use]
#[macro_use]
extern crate error_chain;
extern crate mongodb;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;


mod db;
mod sessions;
pub mod traits;
pub mod errors;
pub mod models;
mod endpoints;


fn main() {
    endpoints::server().launch();
}
