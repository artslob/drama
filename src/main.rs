mod config;

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

    let client = reqwest::blocking::Client::new();

    let token_url = "https://www.reddit.com/api/v1/access_token";
    let r = client
        .post(token_url)
        .header("user-agent", config.user_agent.to_string())
        .basic_auth(config.client_id, Some(config.client_secret))
        .body("grant_type=client_credentials")
        .send()?
        .error_for_status()?;
    println!("{}", r.status());
    // println!("{}", r.clone().text()?);

    let token: Token = r.json()?;
    println!("{:?}", token);

    let subreddit_url = "https://oauth.reddit.com/r/redditdev/about";
    let r = client
        .get(subreddit_url)
        .header("user-agent", config.user_agent.to_string())
        .header("authorization", format!("bearer {}", &token.access_token))
        .send()?
        .error_for_status()?;

    println!("{}", r.status());

    // let j: serde_json::Value = r.json()?;
    let j: Data<Subreddit> = r.json()?;
    // println!("{}", serde_json::to_string_pretty(&j)?);
    println!("{:#?}", j);

    Ok(())
}
