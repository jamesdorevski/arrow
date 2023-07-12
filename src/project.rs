use chrono::{DateTime, Local};
use std::fmt;

use crate::repository::{self};

pub struct Project {
    pub id: i64,
    pub name: String,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl Project {
    pub fn new(
        id: i64,
        name: String,
        created: DateTime<Local>,
        updated: DateTime<Local>,
    ) -> Project {
        Project {
            id, // id is generated by db
            name: name,
            created,
            updated,
        }
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}, Name: {}. Created: {}, Updated: {}",
            self.id, self.name, self.created, self.updated
        )
    }
}

struct TablePadding {
    id: usize,
    name: usize,
    created: usize,
    updated: usize,
}

impl TablePadding {
    pub fn default_padding(max_name: usize) -> Self {
        TablePadding {
            id: 4,
            name: max_name,
            created: 26,
            updated: 26,
        }
    }
}

pub fn add(name: String) {
    let mut new_proj = Project::new(0, name, Local::now(), Local::now());
    new_proj.id = repository::save_project(&new_proj);

    println!("Created project {}", new_proj);
}

pub fn list() {
    let projs = repository::get_projects().expect("Error retrieving projects");
    
    let names: Vec<String> = projs.iter().map(|proj| proj.name.to_string()).collect();
    let max_name_len = max_str_len(names);
 
    let padding = TablePadding::default_padding(max_name_len);
    print_table(padding, projs);
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

fn print_table(padding: TablePadding, projs: Vec<Project>) {
    println!(
        "{:<width_id$} {:<width_name$} {:<width_created$} {:<width_updated$}",
        "ID",
        "NAME",
        "CREATED",
        "UPDATED",
        width_id = padding.id,
        width_name = padding.name,
        width_created = padding.created,
        width_updated = padding.updated
    );

    for proj in &projs {
        println!(
            "{:<width_id$} {:<width_name$} {:<width_created$} {:<width_updated$}",
            &proj.id,
            &proj.name,
            &proj.created,
            &proj.updated,
            width_id = padding.id,
            width_name = padding.name,
            width_created = padding.created,
            width_updated = padding.updated
        );
    }
}
