use chrono::{DateTime, Local, TimeZone, Duration};
use rusqlite::{Connection, Result, params};

use crate::model::{Log, Project};

pub struct Repository {
    conn: Connection,
}

impl Repository {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("arrow").unwrap();
        let db_path = xdg_dirs.get_config_home().join("arrow.db");
        let conn = Connection::open(db_path)?;
        Ok(Repository { conn })
    }

    pub fn save_project(&self, proj: &Project) -> Result<u32> {
        self.conn.execute(
            "INSERT INTO projects (name, description, created, updated) VALUES (?1, ?2, ?3, ?4)",
            params![
                proj.name, 
                proj.description, 
                proj.created.timestamp(), 
                proj.created.timestamp()
            ]
        )?;

        Ok(self.conn.last_insert_rowid() as u32)
    }
    
    // // TODO: handle foreign key violation when logs exist
    // pub fn remove_project(&self, id: &u32) {
    //     match self
    //         .conn
    //         .execute("DELETE FROM projects WHERE id = ?1", &[id])
    //     {
    //         Ok(rows) => {
    //             if rows < 1 {
    //                 eprintln!(
    //                     "No project with id {} exists. Please specify an existing project.",
    //                     id
    //                 );
    //             } else {
    //                 println!("Deleted project {}", id);
    //             }
    //         }
    //         Err(err) => panic!("Delete failed: {}", err),
    //     };
    // }

    // pub fn all_projects(&self) -> Result<Vec<Project>> {
    //     let mut stmt = self.conn.prepare(
    //         "SELECT id, name, created, updated
    //         FROM projects",
    //     )?;
    //     let mut rows = stmt.query([])?;

    //     let mut projs: Vec<Project> = Vec::new();
    //     while let Some(row) = rows.next()? {
    //         let created = to_datetime(row.get(2)?);
    //         let updated = to_datetime(row.get(3)?);

    //         projs.push(Project::new(row.get(0)?, row.get(1)?, created, updated, None));
    //     }

    //     Ok(projs)
    // }

    pub fn get_project(&self, id: &u32) -> Result<Project> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created, updated
            FROM projects
            WHERE id = ?1",
        )?;

        let proj = stmt.query_row([id], |row| {
            let created = to_datetime(row.get(3)?);
            let updated = to_datetime(row.get(4)?);

            Ok(Project::new(row.get(0)?, row.get(1)?, row.get(2)?, created, updated, None))
        })?;

        Ok(proj)
    }

    // pub fn get_project_logs(&self, proj_id: &u32) -> Result<Vec<Log>> {
    //     let mut stmt = self.conn.prepare(
    //         "SELECT id, description, start, end, duration
    //         FROM logs
    //         WHERE project_id = ?1",
    //     )?;

    //     let mut rows = stmt.query([proj_id])?;

    //     let mut logs: Vec<Log> = Vec::new();
    //     while let Some(row) = rows.next()? {
    //         let start = to_datetime(row.get(2)?);
    //         let end = to_datetime(row.get(3)?);

    //         logs.push(Log::new(
    //             row.get(0)?,
    //             *proj_id,
    //             row.get(1)?,
    //             start,
    //             end,
    //             row.get(4)?,
    //         ));
    //     }

    //     Ok(logs)
    // }

    // pub fn get_total_duration(&self, id: &u32) -> Result<Duration> {
    //     let mut stmt = self.conn.prepare(
    //         "SELECT SUM(duration) FROM logs WHERE project_id = ?1"
    //     )?;

    //     let total_dur = stmt.query_row([id], |row| {
    //         Ok(Duration::seconds(row.get(0)?))
    //     })?;

    //     Ok(total_dur)
    // }
}

fn to_datetime(timestamp: i64) -> DateTime<Local> {
    Local
        .timestamp_opt(timestamp, 0)
        .single()
        .expect("Failed to read timestamp")
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // project saves successfully 
    // edge cases - empty string; null string; integer
    // check if project saves w/o description

    #[test]
    fn project_should_save_in_db() {
        // Arrange
        let created = Local::now();
        let updated = Local::now();
        
        let project = Project::new(0, "test".to_owned(), Option::Some("hi".to_owned()), created, updated, None);
        
        let mut conn = Connection::open_in_memory().unwrap();
        embedded::migrations::runner().run(&mut conn).unwrap();
        let repo = Repository { conn };

        // Act
        let id = repo.save_project(&project);

        // Assert
        let actual_project = repo.get_project(&id.unwrap()).unwrap();
        
        assert_eq!("test", actual_project.name);
        assert_eq!("hi", actual_project.description.unwrap());
        assert_eq!(created.timestamp(), actual_project.created.timestamp());
        assert_eq!(updated.timestamp(), actual_project.updated.timestamp());
    }
}