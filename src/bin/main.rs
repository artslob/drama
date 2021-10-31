use drama::config;
use drama::reddit;
use drama::reddit::model::{Data, Listing, Post, Subreddit};

fn main() -> drama::Result<()> {
    let config = config::Config::from_env()?;
    println!("{:#?}", config);
    println!("{}", config.user_agent);

    let client = reddit::Client::new(&config)?;

    let json: Data<Subreddit> = client.get("r/redditdev/about")?.json()?;
    println!("{:#?}", json);

    let tail = format!("user/{}/about", config.user_agent.reddit_username);
    let _about_user: serde_json::Value = client.get(&tail)?.json()?;

    let json: Data<Listing<Data<Post>>> = client.get("top")?.json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
