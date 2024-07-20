CREATE TABLE IF NOT EXISTS guilds
(
    guild_id bigint NOT NULL PRIMARY KEY,
    prefix varchar,
    role_assign_id bigint
);
