use chrono::Local;

use crate::model::Project;
use crate::project::repository;

use super::print::TablePadding;

pub fn add(name: &str) {
    let mut new_proj = Project::new(0, name.to_string(), Local::now(), Local::now());
    new_proj.id = repository::save_project(&new_proj);

    println!("Created project {}", new_proj);
}

pub fn list() {
    let projs = repository::get_projects().expect("Error retrieving projects");

    let names: Vec<String> = projs.iter().map(|proj| proj.name.to_string()).collect();
    let max_name_len = max_str_len(names);

    let padding = TablePadding::default_padding(max_name_len);
    super::print::print_table(padding, projs);
}

pub fn remove(id: &i64) {
    repository::remove_proj(id);
}

pub fn get(id: &i64) {
    // TODO: use anyway library to improve error msgs
    let proj = repository::get_project(id).expect("Error retrieving proj");
    let logs = repository::get_project_logs(id).expect("Error retrieving logs");

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
