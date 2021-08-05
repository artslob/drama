use std::fmt::{Display, Formatter};
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    user_agent: UserAgent,
}

impl Config {
    fn from_env() -> crate::Result<Self> {
        let env = "DRAMA_CONFIG_FILE";
        let path = std::env::var_os(env).ok_or(format!(
            "Could not parse config, env variable '{}' not set",
            env
        ))?;
        Self::from_file(path)
    }

    fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let config_file = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(config_file)?)
    }
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
    let config = Config::from_env()?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    Ok(())
}
