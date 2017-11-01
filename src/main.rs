#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod utils;
mod static_files;

fn main() {
    let matches = utils::app().get_matches();
    utils::initialize_logging(matches.occurrences_of("verbosity"));

    rocket::ignite()
        .mount("/", routes![static_files::index])
        .launch();
}
