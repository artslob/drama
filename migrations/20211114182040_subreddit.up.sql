-- Add up migration script here
CREATE TABLE IF NOT EXISTS subreddit
(
    id                 varchar primary key,
    user_id            varchar not null REFERENCES "user" (id) ON DELETE CASCADE,
    display_name       varchar not null,
    header_title       varchar not null,
    name               varchar not null,
    public_description varchar not null,
    subreddit_type     varchar not null,
    subscribers        int     not null,
    title              varchar not null,
    url                varchar not null
);
