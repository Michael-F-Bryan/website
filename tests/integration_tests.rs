extern crate env_logger;
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde_json;
#[macro_use]
extern crate website;

mod helpers;

use helpers::Docker;
use website::db::{self, DatabaseContents, DbConn};
use website::prelude::*;
use website::errors::*;
use website::times::{TimeSheetEntry, Times};
use rand::Rng;

fn dump_db(conn: &DbConn) -> Result<DatabaseContents> {
    let mut buffer = Vec::new();
    conn.dump_database(&mut buffer).unwrap();

    serde_json::from_slice(&buffer).map_err(|e| e.into())
}

#[test]
fn round_trip_loading_and_dumping() {
    let db = Docker::new().unwrap();
    let mut conn = DbConn(db::connect(db.database_url()).unwrap());
    let original = DatabaseContents {
        users: rand::thread_rng().gen_iter().take(10).collect(),
        timesheet_entries: rand::thread_rng().gen_iter().take(10).collect(),
    };
    let serialized = serde_json::to_vec(&original).unwrap();

    conn.load_database(&serialized).unwrap();

    let mut got = dump_db(&conn).unwrap();

    // zero out the IDs because they would have been set by the db
    for entry in &mut got.timesheet_entries {
        entry.id = None;
    }
    assert_eq!(got, original);
}

#[test]
fn add_a_user_and_verify_them_afterwards() {
    let db = Docker::new().unwrap();
    let mut conn = DbConn(db::connect(db.database_url()).unwrap());
    let username = "Michael";
    let password = "password1";

    let got = conn.new_user(username, password, true).unwrap();
    let user_id = got.uuid;

    assert_eq!(got.name, username);
    assert!(got.admin);

    // make sure he's actually in the database
    let got = dump_db(&conn).unwrap();
    assert_eq!(got.users.len(), 1);
    assert_eq!(got.users[0].name, username);

    // then try to verify them
    let got = conn.validate_user(username, password).unwrap().unwrap();
    assert_eq!(got.name, username);
    assert_eq!(got.uuid, user_id);

    // an incorrect password
    assert!(
        conn.validate_user(username, "Incorrect password")
            .unwrap()
            .is_none()
    );

    // and a non-existent user
    assert!(
        conn.validate_user("non-existent user", "Password123")
            .unwrap()
            .is_none()
    );
}

#[test]
fn create_a_timesheet_entry_and_delete_it_again() {
    let db = Docker::new().unwrap();
    let mut conn = DbConn(db::connect(db.database_url()).unwrap());

    let entry = TimeSheetEntry::new();

    // save the entry
    conn.save_entry(entry.clone()).unwrap();

    // make sure it's in the summary
    let summary = conn.time_summary().unwrap();
    assert_eq!(summary.len(), 1);
    assert_eq!(summary[0].start, entry.start);

    // then delete it again
    let id = summary[0].id.clone().unwrap();
    conn.delete_entry(id).unwrap();

    // and make sure it's actually gone
    assert_eq!(conn.time_summary().unwrap().len(), 0);
}
