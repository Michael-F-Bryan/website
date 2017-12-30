extern crate rocket;
extern crate website;

use rocket::Rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

/// Set up a dummy server, complete with database and fixtures.
fn dummy_server() -> Rocket {
    website::create_server()
}

#[test]
fn get_the_home_page() {
    let server = dummy_server();
    let client = Client::new(server).unwrap();

    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));

    let body = response.body_string().unwrap();
    assert!(body.contains(">Michael-F-Bryan</h1>"));
    assert!(body.contains("jumbotron"));
}
