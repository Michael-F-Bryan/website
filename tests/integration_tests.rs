extern crate failure;
extern crate rocket;
extern crate website;

use rocket::Rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Cookie, Status};
use failure::Error;
use website::database::PostgresPool;
use website::frontend;

pub fn test_server() -> Result<Rocket, Error> {
    let db = PostgresPool::with_fixtures()?;
    Ok(frontend::create_server().manage(db))
}

#[test]
fn validate_all_endpoints() {
    let endpoints = vec![
        ("/", false),
        ("/login", false),
        ("/times", true),
        ("/times/entries/new", true),
        ("/times/slice/new", true),
        ("/static/master.css", false),
        ("/static/favicon.ico", false),
        ("/resume", false),
    ];

    let server = test_server().unwrap();
    let client = Client::new(server).unwrap();

    for (endpoint, requires_auth) in endpoints {
        if requires_auth {
            validate_authenticated(&client, endpoint);
        } else {
            validate_unauthenticated(&client, endpoint);
        }
    }
}

fn validate_authenticated(client: &Client, endpoint: &str) {
    // make an unauthenticated request
    let response = client.get(endpoint).dispatch();
    assert_eq!(response.status(), Status::Unauthorized, "{}", endpoint);

    // then add an auth cookie
    let req = client.get(endpoint);
    req.inner()
        .cookies()
        .add_private(Cookie::new("username", "admin"));
    // and send the request again
    let response = req.dispatch();
    assert_eq!(response.status(), Status::Ok, "{}", endpoint);
}

fn validate_unauthenticated(client: &Client, endpoint: &str) {
    let req = client.get(endpoint);
    let response = req.dispatch();

    assert_eq!(response.status(), Status::Ok, "{}", endpoint);
}

#[test]
fn get_the_home_page() {
    let server = test_server().unwrap();
    let client = Client::new(server).unwrap();

    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));

    let body = response.body_string().unwrap();
    assert!(body.contains(">Michael-F-Bryan</h1>"));
    assert!(body.contains("jumbotron"));
}

#[test]
fn log_in_as_normal_user() {
    let server = test_server().unwrap();
    let client = Client::new(server).unwrap();

    // the user tries to log in as admin (password: admin)
    let req = client
        .post("/login")
        .header(ContentType::Form)
        .body("username=admin&password=admin");
    let response = req.dispatch();

    // they see that the server redirects them back to "/"
    assert_eq!(response.status(), Status::SeeOther);
    let redirected_to = response.headers().get_one("Location").unwrap();
    assert_eq!(redirected_to, "/");

    // and set a cookie
    let set_cookie = response
        .headers()
        .get_one("Set-Cookie")
        .and_then(|c| c.parse::<Cookie>().ok());

    assert!(set_cookie.is_some());
}

#[test]
fn fetch_the_login_page() {
    let server = test_server().unwrap();
    let client = Client::new(server).unwrap();

    let mut response = client.get("/login").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let body = response.body_string().unwrap();
    assert!(body.contains("<h1>Log In</h1>"));
}
