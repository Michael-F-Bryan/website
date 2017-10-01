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
use rocket::response::Redirect;
use rocket_contrib::Template;

mod db;
use db::LoggedInUser;


fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![home, admin, login, login_form])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn home() -> String {
    String::from("Hello World!")
}

#[get("/admin")]
fn admin(user: LoggedInUser) -> String {
    format!("Hello {}!", user.name)
}

#[derive(Debug, Clone, FromForm)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login", data = "<creds>")]
fn login_form(mut cookies: Cookies, creds: Form<LoginRequest>) -> Redirect {
    println!("User logged in: {:?}", creds);

    let auth = Cookie::build(db::AUTH_COOKIE, "1").secure(true).finish();

    cookies.add_private(auth);
    Redirect::to("/")
}

#[get("/login")]
fn login() -> Template {
    let ctx = json!({"title": "Login"});
    Template::render("base", ctx)
}
