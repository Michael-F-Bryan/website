extern crate dotenv;
extern crate failure;
extern crate website;

use std::process;
use std::env;
use website::database::PostgresPool;
use failure::Error;

fn main() {
    dotenv::dotenv().ok();

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        eprintln!("Usage: {}", args[0]);
        eprintln!();
        eprintln!("This program doesn't take any arguments,");
        eprintln!("\tuse environment variables instead.");
        eprintln!();
        eprintln!("DATABASE_URL: String to use when connecting to the database");
        process::exit(1);
    }

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
    } else {
        return Err(failure::err_msg(
            "No database specified. Please set the DATABASE_URL variable",
        ));
    }

    // the server runs indefinitely, therefore if it ever exits there was
    // an error
    Err(Error::from(server.launch()))
}
