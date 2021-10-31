use drama::reddit::model::{Data, User};
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

async fn insert_token(pool: &sqlx::PgPool) -> Result<Token, sqlx::Error> {
    let mut tx = pool.begin().await?;
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
    .fetch_one(&mut tx)
    .await?;
    tx.rollback().await?;
    Ok(token)
}

async fn create_user(
    pool: &sqlx::PgPool,
    config: &drama::config::Config,
) -> Result<(), drama::Error> {
    let token = sqlx::query_as::<_, Token>("SELECT * FROM token LIMIT 1")
        .fetch_one(pool)
        .await
        .map_err(|_| drama::Error::from("could not select token"))?;
    // println!("{:?}", token);
    let user: User = reqwest::Client::builder()
        .user_agent(config.user_agent.to_string())
        .build()?
        .get("https://oauth.reddit.com/api/v1/me")
        .bearer_auth(token.access_token)
        .send()
        .await?
        .json()
        .await?;
    println!("{:?}", user);
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO "user" (id, accept_followers, has_subscribed, has_verified_email,
        hide_from_robots, is_employee, is_gold, is_mod, name,
        total_karma, link_karma, awardee_karma, awarder_karma, comment_karma, verified)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#,
    )
    .bind(user.id)
    .bind(user.accept_followers)
    .bind(user.has_subscribed)
    .bind(user.has_verified_email)
    .bind(user.hide_from_robots)
    .bind(user.is_employee)
    .bind(user.is_gold)
    .bind(user.is_mod)
    .bind(user.name)
    .bind(user.total_karma)
    .bind(user.link_karma)
    .bind(user.awardee_karma)
    .bind(user.awarder_karma)
    .bind(user.comment_karma)
    .bind(user.verified)
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), drama::Error> {
    let config = drama::config::Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect("postgres://drama_user:drama_pass@localhost:5932/drama_db")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);

    // let token: Token = insert_token(&pool).await?;
    // println!("{:#?}", token);

    create_user(&pool, &config).await?;

    Ok(())
}
