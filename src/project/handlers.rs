use chrono::Local;

use crate::model::Project;

use super::repository::Repository;
use super::print::TablePadding;

fn repo_conn() -> Repository {
    Repository::new().expect("Failed to connect to repository!")
}

pub fn add(name: &str) {
    let mut new_proj = Project::new(0, name.to_string(), Local::now(), Local::now());
    let repo = repo_conn();

    new_proj.id = repo.save_project(&new_proj).expect("Failed to create new project!");

    println!("Created project {}", new_proj);
}

pub fn list() {
    let repo = repo_conn();
    let projs = repo.all_projects().expect("Error retrieving your projects");

    let names: Vec<String> = projs.iter().map(|proj| proj.name.to_string()).collect();
    let max_name_len = max_str_len(names);

    let padding = TablePadding::default_padding(max_name_len);
    super::print::print_table(padding, projs);
}

pub fn remove(id: &i64) {
    let repo = repo_conn();
    repo.remove_project(id);
}

pub fn get(id: &i64) {
    let conn = repo_conn();

    // TODO: use anyway library to improve error msgs
    let proj = conn.get_project(id).expect("Error retrieving project!");
    let logs = conn.get_project_logs(id).expect("Error retrieving logs!");

    println!("{}", proj);

    for l in logs {
        println!("{}", l);
    }
}

fn max_str_len(input: Vec<String>) -> usize {
    let mut max_len = 0usize;
    for s in input {
        let len = s.len();

        if len > max_len {
            max_len = len;
        }
    }

    max_len
}
