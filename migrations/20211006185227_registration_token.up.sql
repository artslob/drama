-- Add up migration script here
CREATE TABLE IF NOT EXISTS registration_token
(
    uuid          uuid primary key,
    access_token  varchar not null,
    refresh_token varchar not null,
    token_type    varchar not null,
    expires_in    int     not null,
    scope         varchar not null
);
