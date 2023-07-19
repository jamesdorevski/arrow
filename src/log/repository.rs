use rusqlite::{params, Connection};

use crate::model::Log;

pub fn save(project_id: &u32, log: &Log) -> u32 {
    let conn = Connection::open("arrow.db").expect("Failed to open db");

    // TODO: convert into migration step. Store a local bool somewhere?
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY,
            project_id INTEGER NOT NULL, 
            description TEXT NOT NULL,
            start INTEGER NOT NULL,
            end INTEGER NOT NULL,
            duration INTEGER NOT NULL,

            FOREIGN KEY (project_id) REFERENCES projects(id)
        )",
        (),
    )
    .expect("Failed to execute query");

    conn.execute("PRAGMA foreign_keys = ON", ())
        .expect("Failed to enable foreign keys");

    conn.execute(
        "INSERT INTO logs (project_id, description, start, end, duration)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            &project_id,
            &log.description,
            &log.start.timestamp(),
            &log.end.timestamp(),
            &log.duration.num_seconds(),
        ],
    )
    .expect("Failed to add new log!");

    conn.last_insert_rowid() as u32
}
