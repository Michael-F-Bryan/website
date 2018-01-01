extern crate website;

use website::database::PostgresPool;

#[test]
fn create_a_user() {
    let pool = PostgresPool::temporary().unwrap();
    let conn = pool.new_connection().unwrap();

    assert_eq!(conn.num_users().unwrap(), 0);

    let got = conn.create_user("michael", "password", true).unwrap();

    assert_eq!(got.username, "michael");
    assert!(got.is_admin);

    assert_eq!(conn.num_users().unwrap(), 1);
}

#[test]
fn authenticate_user() {
    let pool = PostgresPool::temporary().unwrap();
    let conn = pool.new_connection().unwrap();

    conn.create_user("michael", "password", true).unwrap();

    assert!(
        conn.authenticate_user("michael", "some invalid pw")
            .is_err()
    );
    assert!(conn.authenticate_user("michael", "password").is_ok());
}
