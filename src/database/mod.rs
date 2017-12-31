/// Automatically generated database bindings used with [Diesel].
///
/// This is produced using the Diesel CLI tool:
///
/// ```bash
/// $ diesel print-schema > src/database/schema.rs
/// ```
///
/// [Diesel]: https://diesel.rs/
pub mod schema;
mod models;
mod traits;

pub use self::models::User;
pub use self::traits::Database;
