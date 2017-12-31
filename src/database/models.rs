#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}
