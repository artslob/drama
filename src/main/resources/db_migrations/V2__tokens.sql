CREATE TABLE IF NOT EXISTS refresh_token
(
    created_at    timestamptz not null default NOW(),
    updated_at    timestamptz not null default NOW(),
    id            bigserial   primary key,
    refresh_token varchar     not null CONSTRAINT uq_refresh_token_refresh_token UNIQUE,
    token_type    varchar     not null,
    scope         varchar     not null
);

CREATE TABLE IF NOT EXISTS access_token
(
    created_at   timestamptz not null default NOW(),
    updated_at   timestamptz not null default NOW(),
    id           bigserial   primary key,
    access_token varchar     not null CONSTRAINT uq_access_token_access_token UNIQUE,
    token_type   varchar     not null,
    expires_in   int         not null,
    scope        varchar     not null
);

SELECT manage_updated_at('refresh_token');
SELECT manage_updated_at('access_token');
