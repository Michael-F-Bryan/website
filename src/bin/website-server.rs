extern crate website;

fn main() {
    let server = website::create_server();

    // TODO: connect to the database

    server.launch();
}
