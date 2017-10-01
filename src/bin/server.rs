#[macro_use]
extern crate clap;
extern crate website;


fn main() {
    website::server().launch();
}
