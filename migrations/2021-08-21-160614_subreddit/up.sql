-- Your SQL goes here
CREATE TABLE subreddit
(
    display_name       VARCHAR NOT NULL,
    header_title       VARCHAR NOT NULL,
    id                 VARCHAR NOT NULL PRIMARY KEY,
    name               VARCHAR NOT NULL,
    public_description VARCHAR NOT NULL,
    subreddit_type     VARCHAR NOT NULL,
    subscribers        INTEGER NOT NULL,
    title              VARCHAR NOT NULL,
    url                VARCHAR NOT NULL
)
