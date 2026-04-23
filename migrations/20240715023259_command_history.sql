CREATE TABLE IF NOT EXISTS command_history (
	id INTEGER NOT NULL primary key AUTOINCREMENT,
	user_id INTEGER NOT NULL,
	guild_id INTEGER NOT NULL,
	command_name TEXT NOT NULL,
	executed_at DATETIME NOT NULL
);
CREATE INDEX IF NOT EXISTS command_history_command_name_idx ON command_history (command_name);
CREATE INDEX IF NOT EXISTS command_history_user_id_idx ON command_history (user_id);
