CREATE TABLE logs (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	message TEXT NOT NULL,
	"start" INTEGER NOT NULL,
	"end" INTEGER NOT NULL,
	project_id INTEGER NOT NULL,
	CONSTRAINT logs_projects_FK FOREIGN KEY (project_id) REFERENCES projects(id)
);
