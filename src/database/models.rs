use chrono::NaiveDateTime;

/// A website user.
#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    /// The user ID assigned by the database.
    pub id: i32,
    /// The user's username.
    pub username: String,
    /// The user's password hash.
    pub password_hash: String,
    /// Is this user an administrator?
    pub is_admin: bool,
}

/// A timesheet entry.
#[derive(Queryable, Serialize, Deserialize)]
pub struct TimesheetEntry {
    /// The timesheet entry ID assigned by the database.
    pub id: i32,
    /// Which user is this entry associated with?
    pub user_id: i32,
    /// When the user started work.
    pub start: Option<NaiveDateTime>,
    /// When the user finished work.
    pub end: Option<NaiveDateTime>,
    /// The amount of time spent on break (in seconds).
    pub breaks: f64,
    /// A quick description of what the user did in the morning.
    pub morning: String,
    /// A quick description of what the user did in the afternoon.
    pub afternoon: String,
}
