extern crate dotenv;
extern crate failure;
extern crate website;

use std::process;
use std::env;
use website::database::PostgresPool;
use failure::Error;

fn main() {
    dotenv::dotenv().ok();

    if let Err(e) = run() {
        eprintln!("Error: {}", e);

        for cause in e.causes().skip(1) {
            eprintln!("\tCaused By: {}", cause);
        }

        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut server = website::create_server();

    if let Ok(db_url) = env::var("DATABASE_URL") {
        let pool = PostgresPool::new(db_url)?;
        server = server.manage(pool);
    }

    // the server runs indefinitely, therefore if it ever exits there was
    // an error
    Err(Error::from(server.launch()))
}
