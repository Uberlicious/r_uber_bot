CREATE TABLE IF NOT EXISTS role_assign
(
    id serial primary key,
    channel bigint not null,
    roles bigint[]as
);
