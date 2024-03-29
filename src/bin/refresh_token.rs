use drama::reddit::model::Token;
use reqwest::Client;

#[tokio::main]
async fn main() -> drama::Result<()> {
    let config = drama::config::Config::from_env()?;
    let body = format!(
        "grant_type=refresh_token&refresh_token={}",
        std::env::var("REFRESH_TOKEN").unwrap()
    );
    let token: Token = Client::new()
        .post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .body(body)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{:?}", token);
    Ok(())
}
