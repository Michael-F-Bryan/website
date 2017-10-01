#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod traits;
pub mod errors;
pub mod models;

infer_schema!("dotenv:DATABASE_URL");
