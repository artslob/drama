mod config;

type Error = Box<dyn std::error::Error + Sync + Send>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> crate::Result<()> {
    let config = config::Config::from_env()?;

    println!("{:#?}", config);
    println!("{}", config.user_agent);

    Ok(())
}
