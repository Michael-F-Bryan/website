#[macro_use]
extern crate error_chain;
extern crate rand;
extern crate serde_json;
extern crate website;

mod helpers;

use helpers::Docker;
use website::DbConn;
use website::models::User;
use website::traits::{DataStore, DatabaseContents};
use rand::Rng;


#[test]
fn round_trip_loading_and_dumping() {
    let db = Docker::new().unwrap();
    let mut conn = DbConn(website::connect(db.database_url()).unwrap());
    let original = DatabaseContents {
        users: rand::thread_rng().gen_iter().take(10).collect(),
    };
    let serialized = serde_json::to_vec(&original).unwrap();

    conn.load_database(&serialized).unwrap();

    let mut buffer = Vec::new();
    conn.dump_database(&mut buffer).unwrap();

    let got: DatabaseContents = serde_json::from_slice(&buffer).unwrap();

    assert_eq!(got, original);
}
