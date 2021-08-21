#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use schema::subreddit;

mod config;
mod reddit;
mod schema;

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Debug, diesel::Queryable, diesel::Insertable)]
#[table_name = "subreddit"]
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

fn main() -> crate::Result<()> {
    let config = config::Config::from_env()?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    let client = reddit::Reddit::new(&config)?;

    let about_redditdev: Data<Subreddit> = client.get("r/redditdev/about")?.json()?;
    println!("{:#?}", about_redditdev);

    let tail = format!("user/{}/about", config.user_agent.reddit_username);
    let _about_me: serde_json::Value = client.get(&tail)?.json()?;

    let posts: Data<Listing<Data<Post>>> = client.get("top")?.json()?;
    println!("{}", serde_json::to_string_pretty(&posts)?);

    let subreddits: Data<Listing<Data<Subreddit>>> = client.get("subreddits/new")?.json()?;
    println!("{}", serde_json::to_string_pretty(&subreddits)?);

    let subreddits: Vec<Subreddit> = subreddits
        .data
        .children
        .into_iter()
        .map(|x| x.data)
        .collect();

    let con = PgConnection::establish("postgres://drama_user:drama_pass@localhost:5932/drama_db")?;

    use schema::subreddit::dsl::*;

    diesel::insert_into(schema::subreddit::table)
        .values(subreddits)
        .execute(&con)?;

    let results = subreddit.limit(10).load::<Subreddit>(&con)?;
    println!("{:#?}", results);

    Ok(())
}
