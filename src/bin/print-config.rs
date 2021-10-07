fn main() -> drama::Result<()> {
    let config = drama::config::Config::from_env()?;
    println!("{:#?}", config);
    println!("{}", config.user_agent);
    Ok(())
}
