use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[derive(serde::Deserialize, sqlx::FromRow, Debug)]
struct Token {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect("postgres://drama_user:drama_pass@localhost:5932/drama_db")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    let token: Token = sqlx::query_as::<_, Token>(
        "INSERT INTO token (uuid, access_token, refresh_token, token_type, \
    expires_in, scope) VALUES ($1, $2, $3, $4, $5, $6)  \
    RETURNING access_token, refresh_token, token_type, expires_in, scope",
    )
    .bind(uuid::Uuid::new_v4())
    .bind("access")
    .bind("refresh")
    .bind("token type")
    .bind(1i32)
    .bind("scope")
    .fetch_one(&pool)
    .await?;

    println!("{:#?}", token);

    assert_eq!(row.0, 150);

    Ok(())
}
