use std::convert::TryInto;

use http::header::AUTHORIZATION;
use reqwest::blocking::Client as HttpClient;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;

#[derive(serde::Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

pub struct Reddit {
    client: HttpClient,
    api_base_url: String,
}

impl Reddit {
    pub fn new(config: &crate::config::Config) -> crate::Result<Self> {
        let token: Token = HttpClient::new()
            .post(&config.access_token_url)
            .basic_auth(&config.client_id, Some(&config.client_secret))
            .body("grant_type=client_credentials")
            .send()?
            .error_for_status()?
            .json()?;

        let client = HttpClient::builder()
            .user_agent(config.user_agent.to_string())
            .default_headers(Self::default_headers(&token)?)
            .build()?;

        Ok(Reddit {
            client,
            api_base_url: config.api_base_url.clone(),
        })
    }

    fn default_headers(token: &Token) -> crate::Result<HeaderMap> {
        let mut default_headers = HeaderMap::new();
        let bearer = format!("bearer {}", &token.access_token);
        default_headers.insert(AUTHORIZATION, bearer.try_into()?);
        Ok(default_headers)
    }

    pub fn get(&self, tail: &str) -> crate::Result<Response> {
        let url = format!(
            "{}/{}",
            self.api_base_url.trim_end_matches('/'),
            tail.trim_end_matches('/')
        );
        let response = self.client.get(url).send()?.error_for_status()?;
        Ok(response)
    }
}
