use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use chrono::{DateTime, Local};

use crate::{
    model::Log,
    project,
    repository::{Repository, Sqlite},
};

fn repo_conn() -> impl Repository {
    Sqlite::new().expect("Failed to connect to repository!")
}

fn track_work() -> (DateTime<Local>, DateTime<Local>) {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();

    ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    let start = Local::now();
    println!("Started log at {}", start);

    // hold thread until ctrl c is pressed
    while !interrupted.load(Ordering::SeqCst) {}

    let end = Local::now();
    println!("Finished log at {}", end);

    (start, end)
}

pub fn new(proj_name: String, msg: String) {
    let repo = repo_conn();

    match repo.get_project_by_name(&proj_name) {
        Err(e) => match e {
            rusqlite::Error::QueryReturnedNoRows => eprintln!(
                "Project {} not found. Would you like to create it? (Y/N)",
                proj_name
            ),
            _ => eprintln!("Error retrieving project: {}", e),
        },
        Ok(p) => {
            let work_time = track_work();
            let log = Log::new(0, p.id, msg, work_time.0, work_time.1);

            match repo.save_log(&p.id, &log) {
                Err(e) => eprintln!("Failed to save log: {}", e),
                Ok(_) => {
                    println!("Created log {}. Updating project duration...", log.message);
                    project::handlers::update(p.id, None, None);
                }
            }
        }
    };
}

// pub fn save_log(proj_id: &u32, msg: Option<String>, dur: &u16) -> Option<Log> {
//     let repo = Repository::new();

//     let mut log = Log::new_no_timestamp(0, *proj_id, msg, *dur as i64);
//     log.id = repo.save_log(&log).expect("Failed to save log!");

//     // println!("Added log {}", log);
//     Some(log)
// }

// pub fn remove_log(proj_id: &u32, log_id: &u32) -> Option<Log> {
//     let repo = Repository::new();
//     repo.remove_log(proj_id, log_id);
//     None
// }

#[cfg(test)]
mod tests {}
