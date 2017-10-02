#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value, double_parens))]

use std::time::Duration;
use std::path::{Path, PathBuf};
use rocket::{Config, Rocket};
use rocket::config::Environment;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{NamedFile, Responder, Response};
use rocket_contrib::Template;
use dotenv;

use sessions::{Session, SessionManager};
use db;
use errors::*;
use users;


pub fn server_with_config(db_url: &str, cfg: Config, log: bool) -> Result<Rocket> {
    dotenv::dotenv().ok();
    let database = db::connect(db_url)?;
    let session_manager = SessionManager::new();

    let mut all_routes = routes![home, static_files];
    all_routes.extend(users::endpoints());

    Ok(
        Rocket::custom(cfg, log)
            .manage(database)
            .manage(session_manager)
            .mount("/", all_routes)
            .attach(Template::fairing()),
    )
}

pub fn server(db_url: &str) -> Result<Rocket> {
    let env = Environment::active()?;
    let cfg = Config::new(env)?;
    server_with_config(db_url, cfg, env.is_dev())
}

#[get("/")]
fn home(session: Option<Session>) -> Template {
    let ctx = json!({ "user": session.map(|s| s.username) });
    Template::render("index", ctx)
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<Cached<NamedFile>> {
    NamedFile::open(Path::new("static/").join(file))
        .map(|f| Cached::new(Duration::from_secs(3600), f))
        .ok()
}

#[derive(Debug)]
pub struct Cached<T> {
    seconds: u32,
    inner: T,
}

impl<T> Cached<T> {
    pub fn new(duration: Duration, inner: T) -> Cached<T> {
        let seconds = duration.as_secs() as u32;

        Cached { seconds, inner }
    }
}

impl<'a, T> Responder<'a> for Cached<T>
where
    T: Responder<'a>,
{
    fn respond_to(self, request: &Request) -> ::std::result::Result<Response<'a>, Status> {
        use rocket::http::hyper::header::{CacheControl, CacheDirective};

        Response::build()
            .merge(self.inner.respond_to(request)?)
            .header(CacheControl(vec![CacheDirective::MaxAge(self.seconds)]))
            .ok()
    }
}
