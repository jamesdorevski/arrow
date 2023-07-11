use rusqlite::{Connection, Result};

use crate::project::Project;

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
        projs.push(Project::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
        ));
    }

    Ok(projs)
}
