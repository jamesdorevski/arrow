use chrono::Local;

use crate::{
    model::{Log, Project},
    print::table::Table,
    repository::{Repository, Sqlite}, time::duration_hours,
};

fn repo_conn() -> impl Repository {
    Sqlite::new().expect("Failed to connect to repository!")
}

fn calculate_total_duration(logs: &Vec<Log>) -> u32 {
    let mut total = 0;
    for log in logs.iter() {
        total += log.end.timestamp() - log.start.timestamp();
    }

    total as u32
}

fn print_projects(projects: &Vec<Project>) {
    let mut table = Table::new(vec![
        "ID".to_string(),
        "Name".to_string(),
        "Description".to_string(),
        "Created".to_string(),
        "Updated".to_string(),
        "Duration".to_string()
    ]);
    for proj in projects {
        table.add_row(vec![
            proj.id.to_string(),
            proj.name.clone(),
            proj.description.clone().unwrap_or("".to_string()),
            proj.created.to_string(),
            proj.updated.to_string(),
            duration_hours(proj.duration)
        ]);
    }
    table.print(&mut std::io::stdout());
}

/// Create new project.
///
/// # Arguments: 
///
/// * `name` - Name of project
/// * `description` - Optional description of project 
pub fn new(name: String, description: Option<String>) {
    let repo = repo_conn();
    match repo.get_project_by_name(&name) {
        Ok(_) => eprintln!(
            "A project with the name \"{}\" already exists. Skipping...",
            name
        ),
        Err(e) if e == rusqlite::Error::QueryReturnedNoRows => {
            let new_proj = Project::new(name, description, Local::now(), Local::now());

            match repo.save_project(&new_proj) {
                Ok(_) => println!("{} created successfully.\n", new_proj.name),
                Err(e) => eprintln!("Failed to create new project: {}", e),
            }
        }
        Err(e) => eprintln!("Error checking for existing project: {}", e),
    }
}

pub fn list() {
    let repo = repo_conn();

    match repo.all_projects() {
        Ok(projects) => {
            print_projects(&projects);
        }
        Err(e) => eprintln!("Error retrieving your projects: {}", e),
    }
}

pub fn update(id: u32, name: Option<String>, description: Option<String>) {
    let repo = repo_conn();

    match repo.get_project(&id) {
        Ok((mut proj, logs)) => {
            if let Some(new_name) = name {
                proj.name = new_name;
            }
            if let Some(new_desc) = description {
                proj.description = Some(new_desc);
            }

            proj.duration = calculate_total_duration(&logs);

            match repo.update_project(&proj) {
                Ok(updated_rows) => {
                    if updated_rows > 0 {
                        println!("{} updated successfully.\n", proj.name);
                    } else {
                        eprintln!("No project with ID {} was found. Is it the right ID?", id);
                    }
                }
                Err(e) => eprintln!("Failed to update project: {}", e),
            }
        }
        Err(e) => eprintln!("Error retrieving project: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::time::to_datetime;

    use super::*;

    #[test]
    fn calculate_total_duration_with_logs_returns_duration_seconds() {
        // Arrange
        let start1 = to_datetime(1709339260);
        let end1 = to_datetime(1709339270);

        let start2 = to_datetime(1709339280);
        let end2 = to_datetime(1709339290);

        let logs = vec![
            Log::new(1, 0, "Log 1".to_string(), start1, end1),
            Log::new(2, 0, "Log 2".to_string(), start2, end2),
        ];

        // Act
        let result = calculate_total_duration(&logs);

        // Assert
        assert_eq!(result, 20);
    }

    #[test]
    fn calculate_total_duration_project_no_logs_returns_0() {
        // Arrange
        let logs: Vec<Log> = vec![];

        // Act
        let result = calculate_total_duration(&logs);

        // Assert
        assert_eq!(result, 0);
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
