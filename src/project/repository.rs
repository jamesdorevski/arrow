use chrono::{DateTime, Local, TimeZone, Duration};
use rusqlite::{Connection, Result};

use crate::model::{Log, Project};

pub struct Repository {
    conn: Connection,
}

impl Repository {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open("arrow.db")?;
        Ok(Repository { conn })
    }

    pub fn save_project(&self, proj: &Project) -> Result<u32> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created INTEGER NOT NULL,
                updated INTEGER NOT NULL
            )",
            (),
        )?;

        self.conn.execute(
            "INSERT INTO projects (name, created, updated) VALUES (?1, ?2, ?3)",
            &[
                &proj.name,
                &proj.created.timestamp().to_string(),
                &proj.updated.timestamp().to_string(),
            ],
        )?;

        Ok(self.conn.last_insert_rowid() as u32)
    }
    
    // TODO: handle foreign key violation when logs exist
    pub fn remove_project(&self, id: &u32) {
        match self
            .conn
            .execute("DELETE FROM projects WHERE id = ?1", &[id])
        {
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

    pub fn all_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created, updated
            FROM projects",
        )?;
        let mut rows = stmt.query([])?;

        let mut projs: Vec<Project> = Vec::new();
        while let Some(row) = rows.next()? {
            let created = to_datetime(row.get(2)?);
            let updated = to_datetime(row.get(3)?);

            projs.push(Project::new(row.get(0)?, row.get(1)?, created, updated, None));
        }

        Ok(projs)
    }

    pub fn get_project(&self, id: &u32) -> Result<Project> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created, updated
            FROM projects
            WHERE id = ?1",
        )?;

        let proj = stmt.query_row([id], |row| {
            let created = to_datetime(row.get(2)?);
            let updated = to_datetime(row.get(3)?);

            Ok(Project::new(row.get(0)?, row.get(1)?, created, updated, None))
        })?;

        Ok(proj)
    }

    pub fn get_project_logs(&self, proj_id: &u32) -> Result<Vec<Log>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, description, start, end, duration
            FROM logs
            WHERE project_id = ?1",
        )?;

        let mut rows = stmt.query([proj_id])?;

        let mut logs: Vec<Log> = Vec::new();
        while let Some(row) = rows.next()? {
            let start = to_datetime(row.get(2)?);
            let end = to_datetime(row.get(3)?);

            logs.push(Log::new(
                row.get(0)?,
                *proj_id,
                row.get(1)?,
                start,
                end,
                row.get(4)?,
            ));
        }

        Ok(logs)
    }

    pub fn get_total_duration(&self, id: &u32) -> Result<Duration> {
        let mut stmt = self.conn.prepare(
            "SELECT SUM(duration) FROM logs WHERE project_id = ?1"
        )?;

        let total_dur = stmt.query_row([id], |row| {
            Ok(Duration::seconds(row.get(0)?))
        })?;

        Ok(total_dur)
    }
}

fn to_datetime(timestamp: i64) -> DateTime<Local> {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Failed to read timestamp")
}
