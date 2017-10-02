#[macro_use]
extern crate log;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate website;

use std::env;
use std::process;
use std::io::Read;
use std::fs::File;
use clap::{App, Arg, ArgMatches, SubCommand};

use website::errors::*;
use website::DbConn;
use website::traits::*;


fn main() {
    let matches = app().get_matches();
    let globals = parse_global_args(&matches).unwrap();

    website::init_logging(globals.verbosity);
    debug!("App args: {:?}", globals);
    debug!("Subcommand: {:?}", matches.subcommand());

    let conn = backtrace!(website::connect(&globals.database_url));
    let conn = DbConn(conn);

    let ret = match matches.subcommand() {
        ("create-user", Some(args)) => create_user(conn, args),
        ("dump-db", _) => dump_database(conn),
        ("load-db", Some(args)) => load_database(conn, args),
        _ => {
            app().print_help().expect("Couldn't print help message");
            println!();
            Ok(())
        }
    };

    backtrace!(ret);
}

fn dump_database(conn: DbConn) -> Result<()> {
    let mut stdout = ::std::io::stdout();
    conn.dump_database(&mut stdout)
    .chain_err(|| "Couldn't dump the database contents to the console")
}

fn load_database(mut conn: DbConn, args: &ArgMatches) -> Result<()> {
    let mut input: Box<Read> = match args.value_of("in-file") {
        Some(filename) => {
            let f = File::open(filename).chain_err(|| "Unable to open input file")?;
            Box::new(f)
        },
        None => Box::new(::std::io::stdin()),
    };

    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).chain_err(|| "Reading failed")?;

    conn.load_database(&buffer).chain_err(|| "Loading new data into the database failed")
}

fn create_user(mut conn: DbConn, args: &ArgMatches) -> Result<()> {
    let username = args.value_of("username").expect("required field");
    let password = args.value_of("password").expect("required field");
    let is_admin = args.is_present("admin");

    conn.new_user(&username, &password, is_admin)
        .chain_err(|| "Couldn't create a new user")
        .map(|_| ())
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
                .arg(Arg::with_name("username").takes_value(true).required(true).help("The new user's username"))
                .arg(Arg::with_name("password").takes_value(true).required(true).help("The new user's password"))
                .arg(Arg::with_name("admin").short("a").long("admin").help("Make the user an administrator"))
                .about("Create a new user."),
        )
        .subcommand(SubCommand::with_name("list-users").about("List all users."))
        .subcommand(SubCommand::with_name("dump-db").about("Dump the database contents as JSON."))
        .subcommand(SubCommand::with_name("load-db").about("Load data into the database.")
        .arg(Arg::with_name("in-file")
            .short("i")
            .long("in-file")
            .takes_value(true)
            .help("The file to read from (defaults to stdin)"))
        )
}
