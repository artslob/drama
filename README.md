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
```
To install sqlx-cli:
```shell
cargo install sqlx-cli --locked
```
