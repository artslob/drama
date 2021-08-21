CREATE TABLE subreddit
(
    display_name       varchar             NOT NULL,
    header_title       varchar             NOT NULL,
    id                 varchar primary key NOT NULL,
    name               varchar             NOT NULL,
    public_description varchar             NOT NULL,
    subreddit_type     varchar             NOT NULL,
    subscribers        integer             NOT NULL,
    title              varchar             NOT NULL,
    url                varchar             NOT NULL
);
