use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
    pub admin: bool,
}
