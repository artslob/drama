# DRAMA

## Prerequisites

Go to https://old.reddit.com/prefs/apps/ and create web-application.  
Settings:
1. Url - web address of your web application, main entry for user.
2. Redirect url - address where user is redirected after he allows application to get token for reddit.
   This endpoint would get code from reddit; next application can send this code + secret to reddit
   API and get access + refresh tokens.

Copy generated client_id and secret to config file.

## Development

Copy and edit config files:
1. `.env.example` -> `.env`
1. `configs/drama-config-example.yml` -> `configs/drama-config.yml`

To export env variables:
```shell
export $(cat .env.example | xargs)
# or
. include-env.sh
```
To install sqlx-cli:
```shell
cargo install sqlx-cli --locked
```

## Constraints naming convention
```
index:       "ix_%(column_0_N_label)s"
unique:      "uq_%(table_name)s_%(column_0_N_name)s"
check:       "ck_%(table_name)s_%(constraint_name)s"
foreign key: "fk_%(table_name)s_%(column_0_N_name)s_%(referred_table_name)s_%(referred_column_0_N_name)s"
primary key: "pk_%(table_name)s"
```
