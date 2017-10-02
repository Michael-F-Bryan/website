#[macro_use]
extern crate clap;
extern crate dotenv;
#[macro_use]
extern crate website;

use std::env;

use clap::App;


fn main() {
    dotenv::dotenv().ok();
    let _ = app().get_matches();

    let database_url = env::var("DATABASE_URL").unwrap();
    let server = backtrace!(website::server(&database_url));

    server.launch();
}


fn app() -> App<'static, 'static> {
    app_from_crate!()
}
