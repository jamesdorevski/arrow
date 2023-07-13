use chrono::{DateTime, Local, Duration};
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Log {
    id: usize,
    description: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
    duration: Duration
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
    // get diff
    // save to db
    // done6
}
