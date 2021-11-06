-- Add up migration script here
CREATE TABLE IF NOT EXISTS refresh_token
(
    uuid          uuid primary key,
    user_id       varchar not null REFERENCES "user" (id) ON DELETE CASCADE,
    refresh_token varchar not null
        CONSTRAINT uq_refresh_token_refresh_token UNIQUE,
    token_type    varchar not null,
    scope         varchar not null
);

CREATE TABLE IF NOT EXISTS access_token
(
    uuid         uuid primary key,
    user_id      varchar not null REFERENCES "user" (id) ON DELETE CASCADE,
    access_token varchar not null
        CONSTRAINT uq_access_token_access_token UNIQUE,
    token_type   varchar not null,
    expires_in   int     not null,
    scope        varchar not null
);
