#![feature(plugin)]
#![feature(conservative_impl_trait)]
#![plugin(rocket_codegen)]
#![feature(custom_derive, try_from)]

extern crate bcrypt;
#[macro_use]
extern crate bson;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate mongodb;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

extern crate rand;


#[macro_use]
mod macros;
pub mod db;
pub mod sessions;
pub mod errors;
mod endpoints;
pub mod times;
pub mod users;

pub use endpoints::{server, server_with_config};

use std::env;
use log::LogLevel;
use env_logger::LogBuilder;
use chrono::Local;


/// A module re-exporting commonly used traits, intended for use as
/// `use website::prelude::*`.
pub mod prelude {
    pub use users::Auth;
    pub use times::Times;
    pub use db::DataStore;
}


/// Initialize `env_logger` using the provided verbosity level.
pub fn init_logging(verbosity: usize) {
    let log_level = match verbosity {
        0 => LogLevel::Warn,
        1 => LogLevel::Info,
        2 => LogLevel::Debug,
        _ => LogLevel::Trace,
    };

    let mut lb = LogBuilder::new();

    if let Ok(rust_log) = env::var("RUST_LOG") {
        lb.parse(&rust_log);
    }

    lb.filter(Some("website"), log_level.to_log_level_filter())
        .format(|record| {
            format!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init()
        .expect("Couldn't initialize env_logger");
}
