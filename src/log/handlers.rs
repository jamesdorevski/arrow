use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use chrono::Local;

use crate::{model::Log, repository::{Repository, Sqlite}};

fn repo_conn() -> impl Repository {
    Sqlite::new().expect("Failed to connect to repository!")
}

pub fn start_logging(proj: String, msg: Option<String>) {
    todo!();
    // let interrupted = Arc::new(AtomicBool::new(false));
    // let interrupted_clone = interrupted.clone();
    // let repo = repo_conn();

    // let proj_id = match repo.get_project(&proj) {
    //     Ok(id) => id,
    //     Err(e) => {
    //         eprintln!("Error retrieving project: {}", e);
    //         return None;
    //     }
    // };

    // ctrlc::set_handler(move || {
    //     println!("received Ctrl+C!");
    //     interrupted_clone.store(true, Ordering::SeqCst);
    // })
    // .expect("Error setting Ctrl+C handler");

    // let start = Local::now();
    // println!("Started log at {}", start);

    // // hold thread until ctrl c is pressed
    // while !interrupted.load(Ordering::SeqCst) {}

    // let end = Local::now();
    // println!("Finished log at {}", end);

    // let mut log = Log::new(0, *proj_id, msg, start, end);
    // log.id = repo.save_log(&log).expect("Failed to save log!");

    // println!("Created log {}", log.message);
    // Some(log)
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
