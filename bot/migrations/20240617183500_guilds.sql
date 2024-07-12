CREATE TABLE IF NOT EXISTS guilds
(
    guild_id bigint NOT NULL PRIMARY KEY,
    prefix bigint,
    role_assign_id int NOT NULL
);
