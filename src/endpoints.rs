use rocket::{self, Rocket};
use rocket::http::Cookies;
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;
use dotenv;
use std::path::{Path, PathBuf};

use sessions::{Session, SessionManager};
use db::{self, DbConn};
use traits::Auth;


pub fn server() -> Rocket {
    dotenv::dotenv().ok();
    let database = db::connect().unwrap();
    let session_manager = SessionManager::new();

    rocket::ignite()
        .manage(database)
        .manage(session_manager)
        .mount("/", routes![home, admin, login, login_form, static_files])
        .attach(Template::fairing())
}

#[get("/")]
fn home(session: Option<Session>) -> Template {
    println!("Session: {:?}", session);

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
) -> Redirect {
    let creds = creds.into_inner();
    println!("{} logged in", creds.username);

    let user = match db.validate_user(&creds.username, &creds.password) {
        Some(u) => u,
        None => return Redirect::to("/login"),
    };

    let new_session = sm.new_session(&user);

    cookies.add(new_session.cookie());
    Redirect::to("/")
}

#[get("/login")]
fn login() -> Template {
    let ctx = json!({"title": "Login"});
    Template::render("login", ctx)
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
