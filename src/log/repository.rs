use rusqlite::{params, Connection, Result, types, ToSql};

use crate::model::Log;

pub struct Repository {
    conn: Connection,
}

impl Repository {
    pub fn new() -> Self {
        let conn = Connection::open("arrow.db").expect("Failed to connect to repository!");
        Repository { conn }
    }

    pub fn save_log(&self, log: &Log) -> Result<u32> {
        let message: Box <dyn ToSql> = sql_value_or_null(log.message);
        let start_timestmp: Box<dyn ToSql> = sql_value_or_null(log.maybe_get_start_timestamp()); 
        let end_timestmp: Box<dyn ToSql> = sql_value_or_null(log.maybe_get_end_timestamp()); 

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL, 
                message TEXT NOT NULL,
                start INTEGER NOT NULL,
                end INTEGER NOT NULL,
                duration INTEGER NOT NULL,

                FOREIGN KEY (project_id) REFERENCES projects(id)
                )",
                (),
                )?;

        self.conn.execute("PRAGMA foreign_keys = ON", ())?;

        self.conn.execute(
            "INSERT INTO logs (project_id, message, start, end, duration)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &log.proj_id,
                &message,
                &start_timestmp,
                &end_timestmp,
                &log.duration.num_seconds(),
            ],
        )?;       

        Ok(self.conn.last_insert_rowid() as u32)
    }

    // TODO: duplicate code with remove_project
    pub fn remove_log(&self, proj_id: &u32, log_id: &u32) {
        match self
            .conn
            .execute("DELETE FROM logs WHERE project_id = ?1 AND id = ?2", &[proj_id, log_id])
        {
            Ok(rows) => {
                if rows < 1 {
                    eprintln!(
                        "No log with project ID {} and ID {} exists. Please specify an existing log.",
                        proj_id, log_id
                    );
                } else {
                    println!("Deleted log {} from project {}", log_id, proj_id);
                }
            },
            Err(err) => panic!("Delete failed!: {}", err),
        };
    }
}

fn sql_value_or_null<T: ToSql>(arg: Option<T>) -> Box<dyn ToSql> {
    match arg {
        Some(v) => Box::new(v),
        None => Box::new(types::Null)
    }
}

