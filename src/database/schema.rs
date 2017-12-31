table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Text,
        is_admin -> Bool,
    }
}