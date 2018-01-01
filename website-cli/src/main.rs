#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate website;

use std::env;
use std::io;
use std::str::FromStr;
use std::process;
use log::LevelFilter;
use env_logger::Builder;
use structopt::StructOpt;
use failure::Error;
use website::database::{PostgresPool, User};

pub mod embedded_migrations {
    embed_migrations!("../migrations");
    pub use self::embedded_migrations::*;
}

fn main() {
    dotenv::dotenv().ok();

    let cmd = Args::from_args();
    cmd.init_logging();

    debug!("{:?}", cmd);

    if let Err(e) = cmd.execute() {
        warn!("Error: {}", e);

        for cause in e.causes().skip(1) {
            warn!("\tCaused By: {}", cause);
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone, PartialEq, StructOpt)]
pub struct Args {
    #[structopt(short = "d", long = "database-url", help = "String for connecting to a database")]
    database_url: Option<String>,
    #[structopt(short = "v", long = "verbose", help = "Generate more verbose output")]
    verbosity: u64,
    #[structopt(subcommand)] cmd: Cmd,
}

impl Args {
    fn database_url(&self) -> Result<String, Error> {
        self.database_url
            .clone()
            .or_else(|| env::var("DATABASE_URL").ok())
            .ok_or_else(|| {
                failure::err_msg("No database url provided. Please set the DATABASE_URL variable.")
            })
    }

    pub fn execute(&self) -> Result<(), Error> {
        let db_string = self.database_url()?;
        self.cmd.execute(&db_string)
    }

    fn init_logging(&self) {
        let mut builder = Builder::new();

        let level = match self.verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        if let Ok(rust_log) = env::var("RUST_LOG") {
            builder.parse(&rust_log);
        } else {
            builder.filter(None, level);
        }

        builder.init();
    }
}

#[derive(Debug, Clone, PartialEq, StructOpt)]
#[structopt(name = "command")]
pub enum Cmd {
    #[structopt(name = "create-user", about = "Create a new user")]
    CreateUser {
        username: String,
        password: String,
        #[structopt(short = "a", long = "admin", help = "Make the user an admin")] admin: bool,
    },
    #[structopt(name = "list-users", about = "List all the available users")]
    ListUsers {
        #[structopt(short = "f", long = "format", default_value = "plain-text",
                    possible_values_raw = "&[\"plain-text\", \"json\", \"p\", \"j\"]")]
        format: Format,
    },
    #[structopt(name = "run-migrations", about = "Run all pending migrations")] RunMigrations {},
}

impl Cmd {
    pub fn execute(&self, db_string: &str) -> Result<(), Error> {
        let db_pool = PostgresPool::new(db_string)?;
        let conn = db_pool.new_connection()?;

        match *self {
            Cmd::CreateUser {
                ref username,
                ref password,
                admin,
            } => {
                info!("Creating new user {}", username);
                conn.create_user(username, password, admin).map(|_| ())?;
            }

            Cmd::ListUsers { format } => {
                info!("Listing users");
                let users = conn.list_users()?;
                format.print_users(&users)?;
            }

            Cmd::RunMigrations {} => {
                info!("Running pending migrations");
                embedded_migrations::run(conn.inner())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    PlainText,
    Json,
}

impl Format {
    fn print_users(&self, users: &[User]) -> Result<(), Error> {
        match *self {
            Format::Json => {
                let stdout = io::stdout();
                serde_json::to_writer_pretty(stdout.lock(), users)?;
            }
            Format::PlainText => {
                println!("Users");
                println!("-----");
                println!();

                for user in users {
                    print!("{}", user.username);
                    if user.is_admin {
                        println!(" (admin)");
                    } else {
                        println!();
                    }
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.to_lowercase();
        match lowercase.as_str() {
            "plain-text" | "plaintext" | "plain" | "p" => Ok(Format::PlainText),
            "json" | "j" => Ok(Format::Json),
            _ => Err(failure::err_msg(format!("Unknown format, \"{}\"", s))),
        }
    }
}
