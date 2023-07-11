use chrono::{Local, DateTime};

use crate::repository::{*, self};

pub struct Project {
    pub name: String,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: name.to_string(),
            created: Local::now(),
            updated: Local::now(),
        }
    }
}

pub fn add(name: &str) {

    // create project object
    println!("Adding new project {}...", name);

    let new_proj = Project::new(name);
    let id = repository::save_project(&new_proj);

    println!("Created {} with id {}", name, id);
}

