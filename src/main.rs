use serde::{Deserialize, Serialize};
use std::time::Duration;

mod config;
mod reddit;

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug)]
struct Subreddit {
    display_name: String,
    header_title: String,
    id: String,
    name: String,
    public_description: String,
    subreddit_type: String,
    subscribers: i32,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    archived: bool,
    author: String,
    author_fullname: String,
    author_is_blocked: bool,
    author_premium: bool,
    created_utc: f32,
    downs: i32,
    edited: bool,
    gilded: i32,
    hidden: bool,
    hide_score: bool,
    id: String,
    is_created_from_ads_ui: bool,
    is_crosspostable: bool,
    is_meta: bool,
    is_original_content: bool,
    is_reddit_media_domain: bool,
    is_robot_indexable: bool,
    is_self: bool,
    is_video: bool,
    locked: bool,
    name: String,
    num_comments: i32,
    num_crossposts: i32,
    over_18: bool,
    pinned: bool,
    quarantine: bool,
    saved: bool,
    score: i32,
    subreddit_subscribers: i32,
    title: String,
    total_awards_received: i32,
    ups: i32,
    upvote_ratio: f32,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Listing<T> {
    after: Option<String>,
    before: Option<String>,
    children: Vec<T>,
    dist: i32,
    geo_filter: String,
    modhash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data<T> {
    data: T,
    kind: String,
}

#[tokio::main]
async fn main() -> crate::Result<()> {
    let config = config::Config::from_env()?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect("postgres://drama_user:drama_pass@localhost:5932/drama_db")
        .await?;

    // sqlx::query("DELETE FROM table").execute(&pool).await?;

    // Make a simple query to return the given parameter
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.0, 150);

    let qwe = sqlx::query!(r#"SELECT * FROM (VALUES ('Hello world')) t1 (col1) WHERE 1 = 1"#,)
        .fetch_one(&pool)
        .await?;
    println!("{:#?}", qwe.col1);

    let client = reddit::Reddit::new(&config)?;

    let json: Data<Subreddit> = client.get("r/redditdev/about")?.json()?;
    println!("{:#?}", json);

    let tail = format!("user/{}/about", config.user_agent.reddit_username);
    let _json: serde_json::Value = client.get(&tail)?.json()?;

    let _json: Data<Listing<Data<Post>>> = client.get("top")?.json()?;
    // println!("{}", serde_json::to_string_pretty(&json)?);

    let subreddits: Data<Listing<Data<Subreddit>>> = client.get("subreddits/new")?.json()?;
    println!("{}", serde_json::to_string_pretty(&subreddits)?);

    sqlx::query!(
        r#"INSERT INTO subreddit (display_name,
header_title,
id,
name,
public_description,
subreddit_type,
subscribers,
title,
url
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);"#,
        subreddits.data.children[0].data.display_name,
        subreddits.data.children[0].data.header_title,
        subreddits.data.children[0].data.id,
        subreddits.data.children[0].data.name,
        subreddits.data.children[0].data.public_description,
        subreddits.data.children[0].data.subreddit_type,
        subreddits.data.children[0].data.subscribers,
        subreddits.data.children[0].data.title,
        subreddits.data.children[0].data.url,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
