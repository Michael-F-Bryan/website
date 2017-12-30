use std::sync::Arc;

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
mod users;

pub use self::users::{NewUser, User};

pub trait Database {}

impl<D: Database> Database for Arc<D> {}
