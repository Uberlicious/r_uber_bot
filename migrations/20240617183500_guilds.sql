CREATE TABLE IF NOT EXISTS guilds
(
    guild_id INTEGER NOT NULL PRIMARY KEY,
    prefix TEXT,
    role_assign_id INTEGER
);
