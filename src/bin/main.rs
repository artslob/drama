use serde::{Deserialize, Serialize};

use drama::config;
use drama::reddit;

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

fn main() -> drama::Result<()> {
    let config = config::Config::from_env()?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    let client = reddit::Reddit::new(&config)?;

    let json: Data<Subreddit> = client.get("r/redditdev/about")?.json()?;
    println!("{:#?}", json);

    let tail = format!("user/{}/about", config.user_agent.reddit_username);
    let _about_user: serde_json::Value = client.get(&tail)?.json()?;

    let json: Data<Listing<Data<Post>>> = client.get("top")?.json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}