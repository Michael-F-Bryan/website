//! Authentication and user management.

use std::ops::Deref;
use rocket::{Outcome, Rocket};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies, Status};
use database::{Database, Postgres};
use log;

/// All authentication endpoints.
pub fn mount_endpoints(r: Rocket) -> Rocket {
    r.mount(
        "/",
        routes![login, login_authenticated, submit_login, logout],
    )
}

#[derive(Serialize, FromForm)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[get("/login", rank = 0)]
pub fn login_authenticated(user: LoggedInUser) -> Template {
    Template::render("login_page", json!{{"username": user.as_ref()}})
}

#[get("/login", rank = 1)]
pub fn login() -> Template {
    Template::render("login_page", json!{{"username": null}})
}

#[get("/logout")]
pub fn logout(user: LoggedInUser, mut cookies: Cookies) -> Redirect {
    if let Some(c) = cookies.get_private("username") {
        log::info!("{} logged out", user.0);
        cookies.remove_private(c);
    }

    Redirect::to("/")
}

#[post("/login", data = "<req>")]
pub fn submit_login(req: Form<LoginRequest>, conn: Postgres, mut cookies: Cookies) -> Redirect {
    let lr = req.into_inner();

    match conn.authenticate_user(&lr.username, &lr.password) {
        Ok(user) => {
            log::info!("{} logged in", user.username);

            let auth_token = Cookie::new("username", user.username);
            cookies.add_private(auth_token);
            Redirect::to("/")
        }
        Err(e) => {
            log::warn!("Failed login for {:?} ({})", lr.username, e);
            Redirect::to("/login")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggedInUser(pub String);

impl LoggedInUser {
    pub fn new<U: Into<String>>(username: U) -> LoggedInUser {
        LoggedInUser(username.into())
    }

    pub fn from_cookies(mut cookies: Cookies) -> Option<LoggedInUser> {
        cookies
            .get_private("username")
            .map(|cookie| LoggedInUser::new(cookie.value()))
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for LoggedInUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<LoggedInUser, ()> {
        match LoggedInUser::from_cookies(request.cookies()) {
            Some(user) => Outcome::Success(user),
            None => Outcome::Forward(()),
        }
    }
}

impl AsRef<str> for LoggedInUser {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

/// A wrapper request guard which will *require* the inner guard to succeed,
/// returning a 401 Unauthorized otherwise.
pub struct LoginRequired<T>(pub T);

impl<'a, 'r, T: FromRequest<'a, 'r>> FromRequest<'a, 'r> for LoginRequired<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match T::from_request(request) {
            Outcome::Success(s) => Outcome::Success(LoginRequired(s)),
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl<T> Deref for LoginRequired<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
