extern crate website;

fn main() {
    let server = website::create_server();
    server.launch();
}
