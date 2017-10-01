use std::convert::TryFrom;
use uuid::Uuid;
use bson::Document;
use errors::*;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
    pub admin: bool,
}

impl From<User> for Document {
    fn from(user: User) -> Document {
        doc!{
            "uuid" => (user.uuid.to_string()),
            "name" => (user.name),
            "password_hash" => (user.password_hash),
            "admin" => (user.admin)
        }
    }
}

impl TryFrom<Document> for User {
    type Error = Error;

    fn try_from(doc: Document) -> Result<Self> {
        let uuid = doc.get_str("uuid")
            .chain_err(|| "Couldn't fetch the 'uuid' field")?;
        let uuid = uuid.parse().chain_err(|| "Invalid UUID format")?;
        let name = doc.get_str("name")
            .chain_err(|| "Couldn't fetch the 'name' field")?
            .to_string();
        let password_hash = doc.get_str("password_hash")
            .chain_err(|| "Couldn't fetch the 'password_hash' field")?
            .to_string();
        let admin = doc.get_bool("admin")
            .chain_err(|| "Couldn't fetch the 'admin' field")?;

        Ok(User {
            uuid,
            name,
            password_hash,
            admin,
        })
    }
}
