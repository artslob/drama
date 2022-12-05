CREATE TABLE IF NOT EXISTS "users"
(
    created_at   timestamptz not null default NOW(),
    updated_at   timestamptz not null default NOW(),
    id VARCHAR PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    accept_followers BOOLEAN NOT NULL,
    has_subscribed BOOLEAN NOT NULL,
    has_verified_email BOOLEAN NOT NULL,
    hide_from_robots BOOLEAN NOT NULL,
    is_employee BOOLEAN NOT NULL,
    is_gold BOOLEAN NOT NULL,
    is_mod BOOLEAN NOT NULL,
    total_karma INTEGER NOT NULL,
    link_karma INTEGER NOT NULL,
    awardee_karma INTEGER NOT NULL,
    awarder_karma INTEGER NOT NULL,
    comment_karma INTEGER NOT NULL,
    verified BOOLEAN NOT NULL
);

SELECT manage_updated_at('users');
