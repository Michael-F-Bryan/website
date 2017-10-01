#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate website;

use std::env;
use clap::{App, Arg, ArgMatches, SubCommand};
use website::errors::*;


fn main() {
    let matches = app().get_matches();
    let globals = parse_global_args(&matches).unwrap();

    website::init_logging(globals.verbosity);
    debug!("App args: {:?}", globals);
    debug!("Subcommand: {:?}", matches.subcommand());

    match matches.subcommand() {
        ("create-user", args) => println!("{:?}", args),
        _ => {
            app().print_help().expect("Couldn't print help message");
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct GlobalArgs {
    database_url: String,
    verbosity: usize,
}

fn parse_global_args(matches: &ArgMatches) -> Result<GlobalArgs> {
    dotenv::dotenv().ok();

    let database_url = matches
        .value_of("database-url")
        .map(|d| d.to_string())
        .or_else(|| env::var("DATABASE_URL").ok());

    let database_url = match database_url {
        Some(d) => d,
        None => bail!("database URL not specified, use the `--database-url` flag or `DATABASE_URL` environment variable.")
    };

    Ok(GlobalArgs {
        database_url: database_url,
        verbosity: matches.occurrences_of("verbosity") as usize,
    })
}

fn app() -> App<'static, 'static> {
    app_from_crate!()
        .bin_name("website-cli")
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .multiple(true),
        )
        .arg(
            Arg::with_name("database-url")
                .short("d")
                .long("database-url")
                .takes_value(true)
                .help("The url for MongoDB (overrides the `DATABASE_URL` env variable)"),
        )
        .subcommand(
            SubCommand::with_name("create-user")
                .arg(Arg::with_name("name").takes_value(true).required(true))
                .about("Create a new user."),
        )
        .subcommand(SubCommand::with_name("list-users").about("List all users."))
}
