use chrono::Local;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::repository::Repository;

use crate::model::Log;

pub fn start_logging(proj_id: &u32, msg: Option<String>) -> Option<Log> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    let repo = Repository::new();

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

    let mut log = Log::new(0, *proj_id, msg, start, end, None);
    log.id = repo.save_log(&log).expect("Failed to save log!");

    // println!("Created log {}", log);
    Some(log)
}

pub fn save_log(proj_id: &u32, msg: Option<String>, dur: &u16) -> Option<Log> {
    let repo = Repository::new();

    let mut log = Log::new_no_timestamp(0, *proj_id, msg, *dur as i64);
    log.id = repo.save_log(&log).expect("Failed to save log!");
    
    // println!("Added log {}", log);
    Some(log)
}

pub fn remove_log(proj_id: &u32, log_id: &u32) -> Option<Log> {
    let repo = Repository::new();
    repo.remove_log(proj_id, log_id);
    None
}
