use drama::config;
use drama::reddit;
use drama::reddit::model::{Data, Listing, Post, Subreddit};

fn main() -> drama::Result<()> {
    let config = config::Config::from_env()?;
    println!("{:#?}", config);
    println!("{}", config.user_agent);

    let client = reddit::Client::new(&config)?;

    let about_redditdev: Data<Subreddit> = client.get("r/redditdev/about")?.json()?;
    println!("{:#?}", about_redditdev);

    let tail = format!("user/{}/about", config.user_agent.reddit_username);
    let about_user: serde_json::Value = client.get(&tail)?.json()?;
    println!("{}", about_user);

    let json: Data<Listing<Data<Post>>> = client.get("top")?.json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    let about_user: serde_json::Value = client.get("subreddits/mine/subscriber")?.json()?;
    println!("{}", about_user);

    Ok(())
}
