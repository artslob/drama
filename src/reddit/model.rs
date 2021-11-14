use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subreddit {
    id: String,
    display_name: String,
    header_title: Option<String>,
    name: String,
    public_description: String,
    subreddit_type: String,
    subscribers: i32,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
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
pub struct User {
    pub id: String,
    pub accept_followers: bool,
    pub has_subscribed: bool,
    pub has_verified_email: bool,
    pub hide_from_robots: bool,
    pub is_employee: bool,
    pub is_gold: bool,
    pub is_mod: bool,
    pub name: String,
    pub total_karma: i32,
    pub link_karma: i32,
    pub awardee_karma: i32,
    pub awarder_karma: i32,
    pub comment_karma: i32,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Listing<T> {
    after: Option<String>,
    before: Option<String>,
    children: Vec<T>,
    dist: i32,
    geo_filter: String,
    modhash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data<T> {
    data: T,
    kind: String,
}
