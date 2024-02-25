use chrono::Local;

use crate::{
    model::Project,
    print::table::Table,
    repository::{Repository, Sqlite},
};

fn repo_conn() -> impl Repository {
    Sqlite::new().expect("Failed to connect to repository!")
}

pub fn new(name: String, description: Option<String>) {
    let repo = repo_conn();
    match repo.get_project_by_name(&name) {
        Ok(_) => eprintln!(
            "A project with the name \"{}\" already exists. Skipping...",
            name
        ),
        Err(e) if e == rusqlite::Error::QueryReturnedNoRows => {
            let new_proj = Project::new(0, name, description, Local::now(), Local::now());

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

pub fn edit(id: u32, name: Option<String>, description: Option<String>) {
    let repo = repo_conn();

    match repo.get_project(&id) {
        Ok((mut proj, _)) => {
            if let Some(new_name) = name {
                proj.name = new_name;
            }
            if let Some(new_desc) = description {
                proj.description = Some(new_desc);
            }

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

fn print_projects(projects: &Vec<Project>) {
    let mut table = Table::new(vec![
        "ID".to_string(),
        "Name".to_string(),
        "Description".to_string(),
        "Created".to_string(),
        "Updated".to_string(),
    ]);
    for proj in projects {
        table.add_row(vec![
            proj.id.to_string(),
            proj.name.clone(),
            proj.description.clone().unwrap_or("".to_string()),
            proj.created.to_string(),
            proj.updated.to_string(),
        ]);
    }
    table.print(&mut std::io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;
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
