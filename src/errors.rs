//! Error types used in this crate.

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Json(::serde_json::Error);
        Bcrypt(::bcrypt::BcryptError);
    }
}
