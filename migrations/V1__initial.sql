CREATE TABLE projects (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	description TEXT,
	created INTEGER NOT NULL,
	updated INTEGER NOT NULL,
	duration INTEGER NOT NULL
);
