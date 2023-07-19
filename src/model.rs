use chrono::{DateTime, Duration, Local};
use std::fmt;

pub struct Project {
    pub id: u32,
    pub name: String,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl Project {
    pub fn new(
        id: u32,
        name: String,
        created: DateTime<Local>,
        updated: DateTime<Local>,
    ) -> Project {
        Project {
            id, // id is generated by db
            name,
            created,
            updated,
        }
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}, Name: {}. Created: {}, Updated: {}",
            self.id, self.name, self.created, self.updated
        )
    }
}

pub struct Log {
    pub id: u32,
    pub proj_id: u32,
    pub description: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub duration: Duration,
}

impl Log {
    pub fn new(
        id: u32,
        proj_id: u32,
        description: String,
        start: DateTime<Local>,
        end: DateTime<Local>,
        duration: Option<i64>,
    ) -> Self {
        let duration = match duration {
            Some(val) => Duration::seconds(val),
            None => Duration::seconds(end.timestamp() - start.timestamp()),
        };

        Log {
            id, // db-generated
            proj_id,
            description,
            start,
            end,
            duration,
        }
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Id: {}, Descrip: {}, Start: {}, End: {}, Duration: {}",
            self.id, self.description, self.start, self.end, self.duration
        )
    }
}
