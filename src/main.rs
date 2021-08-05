use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    user_agent: UserAgent,
}

#[derive(Debug, Deserialize)]
struct UserAgent {
    platform: Option<String>,
    app_id: Option<String>,
    version: Option<String>,
    reddit_username: String,
}

impl Display for UserAgent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let os = std::env::consts::OS;
        let version = env!("CARGO_PKG_VERSION");
        let repository = env!("CARGO_PKG_REPOSITORY");
        let app_id = repository.split("://").last().unwrap_or_default();
        let app_id = if app_id.is_empty() {
            env!("CARGO_PKG_NAME")
        } else {
            app_id
        };
        write!(
            f,
            "{}:{}:{} (by /u/{})",
            self.platform.as_ref().unwrap_or(&os.into()),
            self.app_id.as_ref().unwrap_or(&app_id.into()),
            self.version.as_ref().unwrap_or(&version.into()),
            self.reddit_username
        )
    }
}

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> crate::Result<()> {
    // TODO accept config path as env variable
    let config_path = "configs/drama-config.yml";
    let config_file = std::fs::File::open(config_path)?;
    let config: Config = serde_yaml::from_reader(config_file)?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    Ok(())
}
