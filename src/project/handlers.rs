use chrono::{Local};
use std::io::{self, Write};

use crate::{model::Project, repository::Repository};

fn repo_conn() -> Repository {
    Repository::new().expect("Failed to connect to repository!")
}

pub fn new(name: String, description: Option<String>) -> Result<String, rusqlite::Error> {
    let new_proj = Project::new(0, name, description, Local::now(), Local::now());
    let repo = repo_conn();

    repo.save_project(&new_proj)?;
    Ok(new_proj.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_saved_successfully_should_print_project_name() {
        // Arrange
        // Act
        let res = new("Test Project".to_string(), None);

        // Assert
        assert_eq!("Test Project", res.unwrap());
    }
}


// pub fn list() {
//     let repo = repo_conn();
//     let projects = repo.all_projects().expect("Error retrieving your projects");

//     print_table(&projects);
// }

// pub fn remove(id: &u32) {
//     let repo = repo_conn();
//     repo.remove_project(id);
// }

// pub fn get(id: &u32) {
//     let conn = repo_conn();

//     // TODO: use anyway library to improve error msgs
//     let proj = conn.get_project(id).expect("Error retrieving project!");
//     let logs = conn.get_project_logs(id).expect("Error retrieving logs!");

//     print_table(&vec![proj]);
//     log::print::print_table(&logs);
// }

// // TODO: shouldn't this return an Option? 
// fn calculate_total_duration(id: &u32) -> Duration {
//     let conn = repo_conn();
//     conn.get_total_duration(id).expect("Failed to get total duration")
// }

