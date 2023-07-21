use chrono::{Local, Duration};

use crate::model::Project;

use super::repository::Repository;
use super::print::print_table;

fn repo_conn() -> Repository {
    Repository::new().expect("Failed to connect to repository!")
}

pub fn add(name: &str) {
    let mut new_proj = Project::new(0, name.to_string(), Local::now(), Local::now(), None);
    let repo = repo_conn();

    new_proj.id = repo
        .save_project(&new_proj)
        .expect("Failed to create new project!");

    println!("Created project {}", name);
}

pub fn list() {
    let repo = repo_conn();
    let projs = repo.all_projects().expect("Error retrieving your projects");

    print_table(&projs);
}

pub fn remove(id: &u32) {
    let repo = repo_conn();
    repo.remove_project(id);
}

pub fn get(id: &u32) {
    let conn = repo_conn();

    // TODO: use anyway library to improve error msgs
    let proj = conn.get_project(id).expect("Error retrieving project!");
    let logs = conn.get_project_logs(id).expect("Error retrieving logs!");

    //println!("{}", proj);

    for l in logs {
        println!("{}", l);
    }
}

// TODO: shouldn't this return an Option? 
fn calculate_total_duration(id: &u32) -> Duration {
    let conn = repo_conn();
    conn.get_total_duration(id).expect("Failed to get total duration")
}

