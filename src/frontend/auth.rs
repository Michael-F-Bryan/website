//! Authentication and user management.

use rocket::{Outcome, Route};
use rocket::request::{self, Form, FromRequest, Request};
use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies, Status};
use database::{Database, Postgres};
use log;

/// All authentication endpoints.
pub fn routes() -> Vec<Route> {
    routes![login, submit_login]
}

#[derive(Serialize, FromForm)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[get("/login")]
pub fn login(user: Option<LoggedInUser>) -> Template {
    let ctx = super::base_context(user);
    Template::render("login_page", ctx)
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
        Err(_) => {
            log::warn!("Failed login for {}", lr.username);
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
}

impl<'a, 'r> FromRequest<'a, 'r> for LoggedInUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<LoggedInUser, ()> {
        match request.cookies().get_private("username") {
            Some(cookie) => {
                log::debug!("Found auth token for {}", cookie.value());
                Outcome::Success(LoggedInUser::new(cookie.value()))
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl AsRef<str> for LoggedInUser {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
