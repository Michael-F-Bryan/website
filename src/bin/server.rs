#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate website;

use std::env;


fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    website::server(&database_url).launch();
}
