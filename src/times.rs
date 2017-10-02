use std::convert::TryFrom;
use bson::Document;
use bson::oid::ObjectId;
use chrono::{DateTime, TimeZone, Local, Duration};
use rand::{Rand, Rng};
use mongodb::common::WriteConcern;

use db::DbConn;
use errors::*;

pub const TIMESHEET_ENTRY_NAME: &'static str = "timesheet_entries";

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
        debug!("{} entry for {}", if entry.id.is_some() { "Updating" } else {"Saving" }, entry.start.to_rfc2822());

        let write_concerns = WriteConcern {
            j: true,
            fsync: true,
            ..Default::default()
        };
        self.collection(TIMESHEET_ENTRY_NAME).insert_one(entry.into(), Some(write_concerns))?;

        Ok(())
    }

    fn delete_entry(&mut self, id: ObjectId) -> Result<()> {
        let write_concerns = WriteConcern {
            j: true,
            fsync: true,
            ..Default::default()
        };

        let filter = doc!{"_id" => id};
        let delete_result = self.collection(TIMESHEET_ENTRY_NAME).delete_one(filter, Some(write_concerns))?;

        if delete_result.deleted_count == 1 {
            Ok(())
        } else {
            Err("No entry deleted".into())
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSheetEntry {
    pub id: Option<ObjectId>,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    /// The amount of time spent on breaks, in minutes
    pub breaks: u32,
    pub morning: String,
    pub afternoon: String,
}

impl TimeSheetEntry {
    pub fn new() -> TimeSheetEntry {
        let now = Local::now();

        TimeSheetEntry {
            id: None,
            start: now,
            end: now + Duration::hours(8),
            breaks: 0,
            morning: String::new(),
            afternoon: String::new(),
        }
    }
}

impl From<TimeSheetEntry> for Document {
    fn from(entry: TimeSheetEntry) -> Document {
        doc!{
            "_id" => (entry.id.unwrap_or_else(|| ObjectId::new().expect("Couldn't create a new ObjectId"))),
            "start" => (entry.start.to_rfc3339()),
            "end" => (entry.end.to_rfc3339()),
            "breaks" => (entry.breaks),
            "morning" => (entry.morning),
            "afternoon" => (entry.afternoon)
        }
    }
}

impl TryFrom<Document> for TimeSheetEntry {
    type Error = Error;

    fn try_from(doc: Document) -> Result<TimeSheetEntry> {
        macro_rules! get {
            ($doc:expr, $method:ident, $key:expr) => {
                $doc.$method($key).chain_err(|| format!("Couldn't fetch the '{}' field", $key))
            };
        }
        macro_rules! get_str { ($doc:expr, $key:expr) => { get!($doc, get_str, $key) }; }

        let id = Some(get!(doc, get_object_id, "_id")?).cloned();
        let start = get_str!(doc, "start")?;
        let end = get_str!(doc, "end")?;
        let morning = get_str!(doc, "morning")?.to_string();
        let afternoon = get_str!(doc, "afternoon")?.to_string();
        let breaks = get!(doc, get_i32, "breaks")? as u32;

        let start = DateTime::parse_from_rfc3339(start)
            .chain_err(|| "Couldn't parse the 'start' key")?
            .with_timezone(&Local);
        let end = DateTime::parse_from_rfc3339(end)
            .chain_err(|| "Couldn't parse the 'end' key")?
            .with_timezone(&Local);

        Ok(TimeSheetEntry {
            id,
            start,
            end,
            morning,
            afternoon,
            breaks,
        })
    }
}


impl Rand for TimeSheetEntry {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let id = None;

        let start = random_date(rng);
        let breaks = rng.gen_range(0, 60);
        let end = start + Duration::minutes(rng.gen_range(4*60, 9*60));

        let morning = String::new();
        let afternoon = String::new();

        TimeSheetEntry {
            id, start, breaks, end, morning, afternoon
        }
    }
}

fn random_date<R: Rng>(rng: &mut R) -> DateTime<Local> {
    let y = rng.gen_range(2010, 2020);
    let m = rng.gen_range(1, 12);
    let d = rng.gen_range(1, if m != 1 { 30 } else { 27 });

    let h = rng.gen_range(0, 23);
    let min = rng.gen_range(0, 59);
    let s = rng.gen_range(0, 59);

    println!("{}/{}/{} {}:{}:{}", y, m, d, h, min, s);

    Local.ymd(y, m, d).and_hms(h, min, s)
}