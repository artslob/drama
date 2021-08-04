use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    client_id: String,
    client_secret: String,
    user_agent: String,
}

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> crate::Result<()> {
    // TODO accept config path as env variable
    let config_path = "configs/drama-config.yml";
    let config_file = std::fs::File::open(config_path)?;
    let config: Config = serde_yaml::from_reader(config_file)?;

    println!("{:#?}", config);

    Ok(())
}
