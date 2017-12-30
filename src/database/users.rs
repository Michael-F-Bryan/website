use database::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

/// The type used by diesel when creating a new user.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password_hash: String,
    is_admin: bool,
}

impl NewUser {
    fn new(username: String, password: String, is_admin: bool) -> NewUser {
        let password = password.into();
        // TODO: Actually hash the password here
        let hashed = password;

        NewUser {
            username: username.into(),
            password_hash: hashed,
            is_admin: is_admin,
        }
    }

    /// Create a normal user.
    pub fn normal<U: Into<String>, P: Into<String>>(username: U, password: P) -> NewUser {
        NewUser::new(username.into(), password.into(), false)
    }

    /// Create an administrator.
    pub fn admin<U: Into<String>, P: Into<String>>(username: U, password: P) -> NewUser {
        NewUser::new(username.into(), password.into(), true)
    }
}
