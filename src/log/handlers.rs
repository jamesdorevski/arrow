use chrono::{DateTime, Duration, Local};
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::log::*;

pub struct Log {
    pub id: i64,
    pub description: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub duration: Duration,
}

impl Log {
    pub fn new(id: i64, description: String, start: DateTime<Local>, end: DateTime<Local>) -> Self {
        Log {
            id, // db-generated
            description,
            start,
            end,
            duration: end - start,
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

pub fn start_logging(description: String) {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();

    // grab current timestamp
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        interrupted_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let start = Local::now();
    println!("Started log at {}", start);

    // hold thread until ctrl c is pressed
    while !interrupted.load(Ordering::SeqCst) {}

    let end = Local::now();
    println!("Finished log at {}", end);
    // get end timestamp
    let mut log = Log::new(0, description, start, end);
    log.id = repository::save(3, &log);

    println!("Created log {}", log);
    // get diff
    // save to db
    // done6
}
