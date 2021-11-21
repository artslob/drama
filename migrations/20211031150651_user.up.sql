-- Add up migration script here
CREATE TABLE IF NOT EXISTS "user"
(
    created_at         timestamptz not null default NOW(),
    updated_at         timestamptz not null default NOW(),
    -- TODO transform id from base36 to base10?
    id                 varchar primary key,
    accept_followers   boolean     not null,
    has_subscribed     boolean     not null,
    has_verified_email boolean     not null,
    hide_from_robots   boolean     not null,
    is_employee        boolean     not null,
    is_gold            boolean     not null,
    is_mod             boolean     not null,
    name               varchar     not null,
    total_karma        int         not null,
    link_karma         int         not null,
    awardee_karma      int         not null,
    awarder_karma      int         not null,
    comment_karma      int         not null,
    verified           boolean     not null
);
SELECT manage_updated_at('user');
