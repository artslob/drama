use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Subreddit {
    pub id: String,
    pub display_name: String,
    pub header_title: Option<String>,
    pub name: String,
    pub public_description: String,
    pub subreddit_type: String,
    pub subscribers: i32,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub archived: bool,
    pub author: String,
    pub author_fullname: String,
    pub author_is_blocked: bool,
    pub author_premium: bool,
    pub created_utc: f32,
    pub downs: i32,
    pub edited: bool,
    pub gilded: i32,
    pub hidden: bool,
    pub hide_score: bool,
    pub id: String,
    pub is_created_from_ads_ui: bool,
    pub is_crosspostable: bool,
    pub is_meta: bool,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool,
    pub is_robot_indexable: bool,
    pub is_self: bool,
    pub is_video: bool,
    pub locked: bool,
    pub name: String,
    pub num_comments: i32,
    pub num_crossposts: i32,
    pub over_18: bool,
    pub pinned: bool,
    pub quarantine: bool,
    pub saved: bool,
    pub score: i32,
    pub subreddit_subscribers: i32,
    pub title: String,
    pub total_awards_received: i32,
    pub ups: i32,
    pub upvote_ratio: f32,
    pub url: String,
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
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<T>,
    pub dist: i32,
    pub geo_filter: String,
    pub modhash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data<T> {
    pub data: T,
    pub kind: String,
}
