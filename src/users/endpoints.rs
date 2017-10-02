#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value, double_parens))]

use rocket::Route;
use rocket::request::Form;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket_contrib::Template;

use sessions::{Session, SessionManager};
use db::DbConn;
use users::Auth;
use errors::*;

pub fn endpoints() -> Vec<Route> {
    routes![admin, login, login_form]
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
    creds: Option<Form<LoginRequest>>,
    sm: SessionManager,
    db: DbConn,
) -> Result<Redirect> {
    let creds = match creds {
        Some(creds) => creds.into_inner(),
        None => return Ok(Redirect::to("/login")),
    };

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
