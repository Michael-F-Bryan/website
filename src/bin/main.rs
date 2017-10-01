#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate website;

use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;
use std::path::{Path, PathBuf};

mod db;
use db::LoggedInUser;


fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![home, admin, login, login_form, static_files])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn home(user: Option<LoggedInUser>) -> Template {
    println!("User: {:?}", user);
    let name = user.map(|u| u.name.clone());

    let ctx = json!({ "user": name });
    Template::render("index", ctx)
}

#[get("/admin")]
fn admin(user: LoggedInUser) -> Template {
    let ctx = json!({ "user": user.name.clone() });
    Template::render("admin", ctx)
}

#[derive(Debug, Clone, FromForm)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login", data = "<creds>")]
fn login_form(mut cookies: Cookies, creds: Form<LoginRequest>) -> Redirect {
    println!("User logged in: {:?}", creds);

    cookies.add(Cookie::new(db::AUTH_COOKIE, "1"));
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
