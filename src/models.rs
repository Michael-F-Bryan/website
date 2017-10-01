
#[derive(Debug, Clone, Queryable, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub password_hash: String,
    pub admin: bool,
}
