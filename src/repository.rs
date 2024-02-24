use chrono::{DateTime, Local, TimeZone};
use rusqlite::{Connection, Result, params};

use crate::model::{Log, Project};

pub struct Repository {
    conn: Connection,
}

// contract:
// - save project
/*
Contract:
- save project
- save log 
- save tag
- get project
- get all projects
- tag query 
- update project
- update log
- delete project
- delete log 
- delete tag 
*/

impl Repository {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("arrow").unwrap();
        let db_path = xdg_dirs.get_config_home().join("arrow.db");
        let conn = Connection::open(db_path)?;
        Ok(Repository { conn })
    }

    /// Saves the given project to the database
    /// 
    /// # Arguments
    /// 
    /// * `project` - The project to be saved
    pub fn save_project(&self, project: &Project) -> Result<u32> {
        self.conn.execute(
            "INSERT INTO projects (name, description, created, updated) VALUES (?1, ?2, ?3, ?4)",
            params![
                project.name, 
                project.description, 
                project.created.timestamp(), 
                project.created.timestamp()
            ]
        )?;

        Ok(self.conn.last_insert_rowid() as u32)
    }

    /// Saves the log to the database
    /// 
    /// # Arguments
    /// 
    /// * `project_id` - ID of the project to save the log under
    /// * `log` - The log to be saved
    pub fn save_log(&self, project_id: &u32, log: &Log) -> Result<u32> {
        self.conn.execute(
            "INSERT INTO logs (message, start, end, project_id) VALUES (?1, ?2, ?3, ?4)", 
            params![
                log.message,
                log.start.timestamp(),
                log.end.timestamp(),
                project_id
            ]
        )?;

        Ok(self.conn.last_insert_rowid() as u32)
    }
    
    // TODO: handle foreign key violation when logs exis
    /// Delete a project by it's ID. Deletes all logs that are associated with it
    /// 
    /// # Arguments
    /// 
    /// * `id` - ID of the project to delete
    pub fn delete_project(&self, id: &u32) {
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

    /// Retrieve all projects in the database
    pub fn all_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created, updated
            FROM projects",
        )?;
        let mut rows = stmt.query([])?;

        let mut projects: Vec<Project> = Vec::new();
        while let Some(row) = rows.next()? {
            let created = to_datetime(row.get(3)?);
            let updated = to_datetime(row.get(4)?);

            projects.push(Project::new(row.get(0)?, row.get(1)?, row.get(2)?, created, updated));
        }

        Ok(projects)
    }

    /// Retrieve a project, and it's logs with by project ID
    /// 
    /// # Arguments
    /// 
    /// - `id` - ID of the project to retrieve 
    pub fn get_project(&self, id: &u32) -> Result<(Project, Vec<Log>)> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created, updated
            FROM projects
            WHERE id = ?1",
        )?;
        let proj = stmt.query_row([id], |row| {
            let created = to_datetime(row.get(3)?);
            let updated = to_datetime(row.get(4)?);

            Ok(Project::new(row.get(0)?, row.get(1)?, row.get(2)?, created, updated))
        })?;

        stmt = self.conn.prepare(
            "SELECT l.id, l.message, l.start, l.end 
                FROM projects p
                INNER JOIN logs l ON p.id = l.project_id
                WHERE p.id = ?1"
        )?;

        let mut rows = stmt.query([id])?;
        let mut logs: Vec<Log> = Vec::new();

        while let Some(row) = rows.next()? {
            let start = to_datetime(row.get(2)?);
            let end = to_datetime(row.get(3)?);
            
            logs.push(Log::new(row.get(0)?, *id, row.get(1)?, start, end))
        }

        Ok((proj, logs))
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
    // logs save successfully // log without a project is rejected 
    // edge cases - empty string; null string; integer
    // check if project saves w/o description

    fn test_repo() -> Repository {
        let mut conn = Connection::open_in_memory().unwrap();
        embedded::migrations::runner().run(&mut conn).unwrap();
        Repository { conn }
    }

    #[test]
    fn save_project_should_save_in_db() {
        // Arrange
        let created = Local::now();
        let updated = Local::now();
        
        let project = Project::new(0, "test".to_owned(), Option::Some("hi".to_owned()), created, updated);

        let repo = test_repo();

        // Act
        let id = repo.save_project(&project);

        // Assert
        let (actual_project, _) = repo.get_project(&id.unwrap()).unwrap();
        
        assert_eq!("test", actual_project.name);
        assert_eq!("hi", actual_project.description.unwrap());
        assert_eq!(created.timestamp(), actual_project.created.timestamp());
        assert_eq!(updated.timestamp(), actual_project.updated.timestamp());
    }

    #[test]
    fn project_should_save_in_db_no_description() {
        // Arrange
        let name = "test2";
        let created = Local::now();
        let updated = Local::now();
        
        let project = Project::new(0, name.to_owned(), None, created, updated);

        let repo = test_repo();       

        // Act
        let id = repo.save_project(&project);

        // Assert
        let (actual_project, _) = repo.get_project(&id.unwrap()).unwrap();
        
        assert_eq!(name, actual_project.name);
        assert_eq!(None, actual_project.description);
        assert_eq!(created.timestamp(), actual_project.created.timestamp());
        assert_eq!(updated.timestamp(), actual_project.updated.timestamp());
    }

    #[test]
    fn all_projects_should_return_all_projects() {
        // Arrange
        let mut conn = Connection::open_in_memory().unwrap();
        embedded::migrations::runner().run(&mut conn).unwrap();
        let repo = Repository { conn };
        
        for n in 1..=2 {
            let name = format!("test{}", n);
            let created = Local::now();
            let updated = Local::now();
            
            let project = Project::new(0, name.to_owned(), Option::Some("hi".to_owned()), created, updated);
            let _ = repo.save_project(&project);
        }

        // Act
        let projects = repo.all_projects().unwrap();

        // Assert
        assert_eq!(2, projects.len());
    }

    #[test]
    fn save_log_should_save_in_db() {
        // Arrange
        let repo = test_repo();

        let created = Local::now();
        let updated = Local::now();
        let project = Project::new(0, "project".to_owned(), None, created, updated);
        
        let message = "code cleanup";
        let log = Log::new(0, project_id, message.to_owned(), created, updated);
        
        let project_id = repo.save_project(&project).unwrap();
        
        // Act
        repo.save_log(&project_id, &log).unwrap();

        // Assert
        let (actual_project, actual_logs) = repo.get_project(&project_id).unwrap();

        assert_eq!("project", actual_project.name);
        assert_eq!(created.timestamp(), actual_project.created.timestamp());
        assert_eq!(updated.timestamp(), actual_project.updated.timestamp());

        assert_eq!(1, actual_logs.len());
        assert_eq!(message, actual_logs[0].message);
        assert_eq!(created.timestamp(), actual_logs[0].start.timestamp());
        assert_eq!(updated.timestamp(), actual_logs[0].end.timestamp());
    }
}