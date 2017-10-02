#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value, double_parens))]

use std::time::Duration;
use std::path::{Path, PathBuf};
use rocket::{Config, Rocket};
use rocket::config::Environment;
use rocket::http::{Cookies, Status};
use rocket::request::Form;
use rocket::request::Request;
use rocket::response::{NamedFile, Redirect, Responder, Response};
use rocket_contrib::Template;
use dotenv;

use sessions::{Session, SessionManager};
use db::{self, DbConn};
use traits::Auth;
use errors::*;


pub fn server_with_config(db_url: &str, cfg: Config, log: bool) -> Result<Rocket> {
    dotenv::dotenv().ok();
    let database = db::connect(db_url)?;
    let session_manager = SessionManager::new();

    Ok(
        Rocket::custom(cfg, log)
            .manage(database)
            .manage(session_manager)
            .mount("/", routes![home, admin, login, login_form, static_files])
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

#[get("/admin")]
fn admin(session: Session) -> Template {
    let ctx = json!({ "user": session.username.clone() });
    Template::render("admin", ctx)
}

#[derive(Debug, Clone, FromForm)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login", data = "<creds>")]
fn login_form(
    mut cookies: Cookies,
    creds: Form<LoginRequest>,
    sm: SessionManager,
    db: DbConn,
) -> Result<Redirect> {
    let creds = creds.into_inner();
    println!("{} logged in", creds.username);

    let user = match db.validate_user(&creds.username, &creds.password)? {
        Some(u) => u,
        None => return Ok(Redirect::to("/login")),
    };

    let new_session = sm.new_session(&user);

    cookies.add(new_session.cookie());
    Ok(Redirect::to("/"))
}

#[get("/login")]
fn login() -> Template {
    let ctx = json!({"title": "Login"});
    Template::render("login", ctx)
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
