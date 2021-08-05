use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    user_agent: UserAgent,
}

#[derive(Debug, Deserialize)]
struct UserAgent {
    platform: String,
    app_id: Option<String>,
    version: String,
    reddit_username: String,
}

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> crate::Result<()> {
    // TODO accept config path as env variable
    let config_path = "configs/drama-config.yml";
    let config_file = std::fs::File::open(config_path)?;
    let config: Config = serde_yaml::from_reader(config_file)?;
    let user_agent = format!(
        "{}:{}:{} (by /u/{})",
        &config.user_agent.platform,
        // TODO set app_id as cargo.toml:repository field
        &config.user_agent.app_id.as_ref().unwrap_or(&"drama".into()),
        // TODO get from cargo.toml
        &config.user_agent.version,
        &config.user_agent.reddit_username,
    );

    println!("{:#?}", config);
    println!("{}", std::env::consts::OS);
    println!("{}", user_agent);

    Ok(())
}
