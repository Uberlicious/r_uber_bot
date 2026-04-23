CREATE TABLE IF NOT EXISTS managed_roles (
    guild_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    label TEXT NOT NULL,
    emoji TEXT,
    PRIMARY KEY (guild_id, role_id)
);
