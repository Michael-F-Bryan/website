use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct TimesheetEntry {
    pub id: i32,
    pub user_id: i32,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    /// The amount of time spent on break (in seconds).
    pub breaks: f64,
    pub morning: String,
    pub afternoon: String,
}
