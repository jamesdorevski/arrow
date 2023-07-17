use chrono::{DateTime, Local, TimeZone};
use rusqlite::{Connection, Result};

use crate::project::handlers::Project;

pub fn save_project(proj: &Project) -> i64 {
    let conn = Connection::open("arrow.db").expect("Failed to open db");

    conn.execute(
        "create table if not exists projects (
            id integer primary key,
            name text not null,
            created integer not null,
            updated integer not null
        )",
        (),
    )
    .expect("Failed to create table");

    conn.execute(
        "insert into projects (name, created, updated) values (?1, ?2, ?3)",
        &[
            &proj.name,
            &proj.created.timestamp().to_string(),
            &proj.updated.timestamp().to_string(),
        ],
    )
    .expect("Failed to add new project!");

    conn.last_insert_rowid()
}

pub fn get_projects() -> Result<Vec<Project>> {
    let conn = Connection::open("arrow.db").expect("Failed to open db");

    let mut stmt = conn.prepare(
        "SELECT id, name, created, updated
        FROM projects",
    )?;
    let mut rows = stmt.query([])?;

    let mut projs: Vec<Project> = Vec::new();
    while let Some(row) = rows.next()? {
        let created = to_datetime(row.get(2)?);
        let updated = to_datetime(row.get(3)?);

        projs.push(Project::new(row.get(0)?, row.get(1)?, created, updated));
    }

    Ok(projs)
}

pub fn delete_project(id: &usize) {
    let conn = Connection::open("arrow.db").expect("Failed to open db");

    match conn.execute("DELETE FROM projects WHERE id = ?1", &[id]) {
        Ok(rows) => {
            if rows < 1 {
                eprintln!(
                    "No project with id {} exists. Please specify an existing project.",
                    id
                );
            } else {
                println!("Deleted project {}", id);
            }
        }
        Err(err) => panic!("Delete failed: {}", err),
    };
}

pub fn get_project(id: i64) -> Result<Project> {
    let conn = Connection::open("arrow.db").expect("Failed to open db");

    let mut stmt = conn.prepare(
        "SELECT id, name, created, updated
        FROM projects
        WHERE id = ?1",
    )?;

    let proj = stmt.query_row([id], |row| {
        let created = to_datetime(row.get(2)?);
        let updated = to_datetime(row.get(3)?);

        Ok(Project::new(row.get(0)?, row.get(1)?, created, updated))
    })?;

    Ok(proj)
}

fn to_datetime(timestamp: i64) -> DateTime<Local> {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Failed to read timestamp")
}
