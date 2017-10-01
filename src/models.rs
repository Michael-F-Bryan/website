
#[derive(Debug, Clone, Queryable)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub password_hash: String,
    pub admin: bool,
}
