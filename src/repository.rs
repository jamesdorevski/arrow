use chrono::{DateTime, Local, TimeZone};
use rusqlite::{params, Connection, Result};

use crate::model::{Log, Project};

pub trait Repository {
    fn all_projects(&self) -> Result<Vec<Project>>;
    fn save_project(&self, project: &Project) -> Result<u32>;
    fn save_log(&self, project_id: &u32, log: &Log) -> Result<u32>;
    fn get_project(&self, id: &u32) -> Result<(Project, Vec<Log>)>;
    fn get_project_by_name(&self, name: &str) -> Result<Project>;
    fn get_logs(&self, proj_id: &u32, msg: &str) -> Result<Vec<Log>>;
    fn update_project(&self, project: &Project) -> Result<usize>;
    fn delete_project(&self, id: &u32);
    fn delete_log(&self, proj_id: &u32, log_id: &u32);
}

pub struct Sqlite {
    conn: Connection,
}

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

impl Sqlite {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("arrow").unwrap();
        let db_path = xdg_dirs.get_config_home().join("arrow.db");
        let conn = Connection::open(db_path)?;
        Ok(Sqlite { conn })
    }
}

impl Repository for Sqlite {
    /// Saves the given project to the database
    ///
    /// # Arguments
    ///
    /// * `project` - The project to be saved
    fn save_project(&self, project: &Project) -> Result<u32> {
        self.conn.execute(
            "INSERT INTO projects (name, description, created, updated) VALUES (?1, ?2, ?3, ?4)",
            params![
                project.name,
                project.description,
                project.created.timestamp(),
                project.created.timestamp()
            ],
        )?;

        Ok(self.conn.last_insert_rowid() as u32)
    }

    /// Saves the log to the database
    ///
    /// # Arguments
    ///
    /// * `project_id` - ID of the project to save the log under
    /// * `log` - The log to be saved
    fn save_log(&self, project_id: &u32, log: &Log) -> Result<u32> {
        self.conn.execute(
            "INSERT INTO logs (message, start, end, project_id) VALUES (?1, ?2, ?3, ?4)",
            params![
                log.message,
                log.start.timestamp(),
                log.end.timestamp(),
                project_id
            ],
        )?;

        Ok(self.conn.last_insert_rowid() as u32)
    }

    /// Delete a project by it's ID. Deletes all logs that are associated with it
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the project to delete
    fn delete_project(&self, id: &u32) {
        match self
            .conn
            .execute("DELETE FROM logs WHERE project_id = ?1", [id])
        {
            Ok(rows) => {
                if rows < 1 {
                    eprintln!(
                        "No logs for project with id {} exists. Please specify an existing project.",
                        id
                    );
                } else {
                    println!("Deleted logs for project {}", id);
                }
            }
            Err(err) => panic!("Delete failed: {}", err),
        }

        match self
            .conn
            .execute("DELETE FROM projects WHERE id = ?1", [id])
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

    /// Delete a log by it's project ID and log ID.
    ///
    /// # Arguments
    ///
    /// * `proj_id` - ID of the project to delete the log from
    /// * `log_id` - ID of the log to delete
    fn delete_log(&self, proj_id: &u32, log_id: &u32) {
        match self.conn.execute(
            "DELETE FROM logs WHERE id = ?1 AND project_id = ?2",
            [log_id, proj_id],
        ) {
            Ok(rows) => {
                if rows < 1 {
                    eprintln!(
                        "No log with id {} exists for project {}. Please specify an existing log.",
                        log_id, proj_id
                    );
                } else {
                    println!("Deleted log {}", log_id);
                }
            }
            Err(err) => panic!("Delete failed: {}", err),
        };
    }

    /// Retrieve all projects in the database
    fn all_projects(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created, updated
            FROM projects",
        )?;
        let mut rows = stmt.query([])?;

        let mut projects: Vec<Project> = Vec::new();
        while let Some(row) = rows.next()? {
            let created = to_datetime(row.get(3)?);
            let updated = to_datetime(row.get(4)?);

            projects.push(Project::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                created,
                updated,
            ));
        }

        Ok(projects)
    }

    /// Retrieve a project, and it's logs with by project ID
    ///
    /// # Arguments
    ///
    /// - `id` - ID of the project to retrieve
    fn get_project(&self, id: &u32) -> Result<(Project, Vec<Log>)> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created, updated
            FROM projects
            WHERE id = ?1",
        )?;
        let proj = stmt.query_row([id], |row| {
            let created = to_datetime(row.get(3)?);
            let updated = to_datetime(row.get(4)?);

            Ok(Project::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                created,
                updated,
            ))
        })?;

        stmt = self.conn.prepare(
            "SELECT l.id, l.message, l.start, l.end 
                FROM projects p
                INNER JOIN logs l ON p.id = l.project_id
                WHERE p.id = ?1",
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

    fn get_project_by_name(&self, name: &str) -> Result<Project> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created, updated
            FROM projects
            WHERE name = ?1",
        )?;
        let proj = stmt.query_row([name], |row| {
            let created = to_datetime(row.get(3)?);
            let updated = to_datetime(row.get(4)?);

            Ok(Project::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                created,
                updated,
            ))
        })?;

        Ok(proj)
    }

    fn get_logs(&self, proj_id: &u32, msg: &str) -> Result<Vec<Log>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                l.id
                l.message
                l.start
                l.end
            FROM 
                logs l 
            INNER JOIN 
                projects p ON l.project_id = p.id
            WHERE 
                p.id = ?1 
            AND
                l.message LIKE %?2%",
        )?;

        let mut rows = stmt.query(params![proj_id, msg])?;
        let mut logs: Vec<Log> = Vec::new();

        while let Some(row) = rows.next()? {
            let start = to_datetime(row.get(2)?);
            let end = to_datetime(row.get(3)?);

            logs.push(Log::new(row.get(0)?, *proj_id, row.get(1)?, start, end));
        }

        Ok(logs)
    }

    fn update_project(&self, project: &Project) -> Result<usize> {
        match self.conn.execute(
            "UPDATE projects
            SET name = ?1, description = ?2, updated = ?3
            WHERE id = ?4",
            params![
                project.name,
                project.description,
                Local::now().timestamp(),
                project.id
            ],
        ) {
            Ok(updated) => return Ok(updated),
            Err(err) => panic!("Update failed: {}", err),
        }
    }
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

    // project saves successfully  // DONE
    // logs save successfully // log without a project is rejected  // DONE
    // edge cases - empty string; null string; integer
    // check if project saves w/o description // DONE
    // delete projects + logs // DONE
    // update projects + logs

    fn test_repo() -> impl Repository {
        let mut conn = Connection::open_in_memory().unwrap();
        embedded::migrations::runner().run(&mut conn).unwrap();
        Sqlite { conn }
    }

    fn default_test_project() -> Project {
        let created = Local::now();
        let updated = Local::now();

        Project::new(
            0,
            "test".to_owned(),
            Option::Some("hi".to_owned()),
            created,
            updated,
        )
    }

    #[test]
    fn save_project_should_save_in_db() {
        // Arrange
        let repo = test_repo();
        let project = default_test_project();

        // Act
        let id = repo.save_project(&project);

        // Assert
        let (actual_project, _) = repo.get_project(&id.unwrap()).unwrap();

        assert_eq!("test", actual_project.name);
        assert_eq!("hi", actual_project.description.unwrap());
        assert_eq!(
            project.created.timestamp(),
            actual_project.created.timestamp()
        );
        assert_eq!(
            project.updated.timestamp(),
            actual_project.updated.timestamp()
        );
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
        let repo = test_repo();

        for n in 1..=2 {
            let name = format!("test{}", n);
            let created = Local::now();
            let updated = Local::now();

            let project = Project::new(
                0,
                name.to_owned(),
                Option::Some("hi".to_owned()),
                created,
                updated,
            );
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

        let project_id = repo.save_project(&project).unwrap();

        let message = "code cleanup";
        let log = Log::new(0, project_id, message.to_owned(), created, updated);

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

    #[test]
    fn save_log_no_project_should_fail() {
        // Arrange
        let repo = test_repo();
        let start = Local::now();
        let end = Local::now();

        let message = "code cleanup";
        let log = Log::new(0, 0, message.to_owned(), start, end);

        // Act
        let res = repo.save_log(&0, &log);

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn delete_project_should_delete_project_from_db() {
        // Arrange

        let repo = test_repo();
        let project = default_test_project();

        let project_id = repo.save_project(&project).unwrap();

        let message = "code cleanup";
        let log = Log::new(
            0,
            project_id,
            message.to_owned(),
            project.created,
            project.updated,
        );

        repo.save_log(&project_id, &log).unwrap();

        // Act
        repo.delete_project(&project_id);

        // confirm log + project is removed
        // Assert
        let res = repo.get_project(&project_id);
        assert!(res.is_err());
    }

    #[test]
    fn delete_log_should_delete_log_from_db() {
        // Arrange
        let repo = test_repo();
        let project = default_test_project();

        let project_id = repo.save_project(&project).unwrap();

        let message = "code cleanup";
        let log = Log::new(
            0,
            project_id,
            message.to_owned(),
            project.created,
            project.updated,
        );

        let log_id = repo.save_log(&project_id, &log).unwrap();

        // Act
        repo.delete_log(&project_id, &log_id);

        // Assert
        let (_, logs) = repo.get_project(&project_id).unwrap();
        assert_eq!(0, logs.len());
    }

    #[test]
    fn update_project_should_update_project_in_db() {
        // Arrange
        let repo = test_repo();
        let project = default_test_project();

        let project_id = repo.save_project(&project).unwrap();

        let updated_name = "updated";
        let updated_desc = "updated desc";
        let updated_project = Project::new(
            project_id,
            updated_name.to_owned(),
            Option::Some(updated_desc.to_owned()),
            Local::now(),
            Local::now(),
        );

        // Act
        let updated_rows = repo.update_project(&updated_project).unwrap();

        // Assert
        let (actual_project, _) = repo.get_project(&project_id).unwrap();

        assert_eq!(1, updated_rows);
        assert_eq!(updated_name, actual_project.name);
        assert_eq!(updated_desc, actual_project.description.unwrap());
    }

    #[test]
    fn update_project_incorrect_id_should_fail() {
        // Arrange
        let repo = test_repo();
        let project = default_test_project();

        let project_id = repo.save_project(&project).unwrap();

        let updated_name = "updated";
        let updated_desc = "updated desc";
        let updated_project = Project::new(
            project_id + 1,
            updated_name.to_owned(),
            Option::Some(updated_desc.to_owned()),
            Local::now(),
            Local::now(),
        );

        // Act
        let res = repo.update_project(&updated_project).unwrap();

        // Assert
        assert_eq!(0, res);
    }

    #[test]
    fn get_project_by_name_should_return_project() {
        // Arrange
        let repo = test_repo();
        let project = default_test_project();

        let project_id = repo.save_project(&project).unwrap();

        // Act
        let actual_project = repo.get_project_by_name(&project.name).unwrap();

        // Assert
        assert_eq!(project_id, actual_project.id);
    }

    #[test]
    fn get_project_by_name_no_project_found_should_return_error() {
        // Arrange
        let repo = test_repo();

        // Act
        let res = repo.get_project_by_name("not a project");

        // Assert
        assert!(res.is_err());
    }
}
