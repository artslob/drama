mod config;
mod reddit;

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

#[derive(serde::Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
struct Data<T> {
    data: T,
    kind: String,
}

fn main() -> crate::Result<()> {
    let config = config::Config::from_env()?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    let client = reddit::Reddit::new(&config)?;

    let json: Data<Subreddit> = client.get("r/redditdev/about")?.json()?;
    println!("{:#?}", json);

    let tail = format!("user/{}/about", config.user_agent.reddit_username);
    let json: serde_json::Value = client.get(&tail)?.json()?;
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
