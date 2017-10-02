//! Traits and datastructures related to the timesheet module.

mod models;
mod traits;

pub use self::models::TimeSheetEntry;
pub use self::traits::{Times, TIMESHEET_ENTRY_NAME};
