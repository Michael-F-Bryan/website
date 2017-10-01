#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate website;

mod db;
use db::DbConn;


fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![home])
        .launch();
}

#[get("/")]
fn home(database: DbConn) -> String {
    String::from("Hello World!")
}
