CREATE TABLE IF NOT EXISTS role_assign
(
    id bigserial primary key,
    channel bigint not null,
    roles bigint[]
);
