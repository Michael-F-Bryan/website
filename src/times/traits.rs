use bson::oid::ObjectId;
use mongodb::common::WriteConcern;

use db::DbConn;
use super::TimeSheetEntry;
use errors::*;


/// The name of the collection used to store timesheet entries.
pub const TIMESHEET_ENTRY_NAME: &'static str = "timesheet_entries";

/// An interface to the timesheet handling parts of the database.
pub trait Times {
    fn time_summary(&self) -> Result<Vec<TimeSheetEntry>>;
    fn save_entry(&mut self, entry: TimeSheetEntry) -> Result<()>;
    fn delete_entry(&mut self, id: ObjectId) -> Result<()>;
}

impl Times for DbConn {
    fn time_summary(&self) -> Result<Vec<TimeSheetEntry>> {
        let entries: Result<Vec<TimeSheetEntry>> = self.find_all(TIMESHEET_ENTRY_NAME)?.collect();
        let mut entries = entries?;

        for entry in &mut entries {
            entry.morning.clear();
            entry.afternoon.clear();
        }

        Ok(entries)
    }

    fn save_entry(&mut self, entry: TimeSheetEntry) -> Result<()> {
        debug!(
            "{} entry for {}",
            if entry.id.is_some() {
                "Updating"
            } else {
                "Saving"
            },
            entry.start.to_rfc2822()
        );

        let write_concerns = WriteConcern {
            j: true,
            fsync: true,
            ..Default::default()
        };
        self.collection(TIMESHEET_ENTRY_NAME)
            .insert_one(entry.into(), Some(write_concerns))?;

        Ok(())
    }

    fn delete_entry(&mut self, id: ObjectId) -> Result<()> {
        let write_concerns = WriteConcern {
            j: true,
            fsync: true,
            ..Default::default()
        };

        let filter = doc!{"_id" => id};
        let delete_result = self.collection(TIMESHEET_ENTRY_NAME)
            .delete_one(filter, Some(write_concerns))?;

        if delete_result.deleted_count == 1 {
            Ok(())
        } else {
            Err("No entry deleted".into())
        }
    }
}
