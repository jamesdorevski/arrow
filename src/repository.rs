use rusqlite::{Connection};

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
        ()
    ).expect("Failed to create table");

    conn.execute(
        "insert into projects (name, created, updated) values (?1, ?2, ?3)",
        &[&proj.name, &proj.created.timestamp().to_string(), &proj.updated.timestamp().to_string()],
    ).expect("Failed to add new project!");

    conn.last_insert_rowid()
}
