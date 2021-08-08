use reqwest::blocking::Client as HttpClient;
use reqwest::blocking::Response;

#[derive(serde::Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

pub struct Reddit {
    client: HttpClient,
    token: Token,
    authorization: String,
    user_agent: String,
    api_base_url: String,
}

impl Reddit {
    pub fn new(config: &crate::config::Config) -> crate::Result<Self> {
        let client = HttpClient::new();
        let user_agent = config.user_agent.to_string();

        let response = client
            .post(&config.access_token_url)
            .header("user-agent", &user_agent)
            .basic_auth(&config.client_id, Some(&config.client_secret))
            .body("grant_type=client_credentials")
            .send()?
            .error_for_status()?;

        let token: Token = response.json()?;
        let authorization = format!("bearer {}", &token.access_token);

        Ok(Reddit {
            client,
            token,
            authorization,
            user_agent,
            api_base_url: config.api_base_url.clone(),
        })
    }

    pub fn get(&self, tail: &str) -> crate::Result<Response> {
        let url = format!("{}/{}", self.api_base_url, tail);
        let response = self
            .client
            .get(url)
            .header("user-agent", &self.user_agent)
            .header("authorization", &self.authorization)
            .send()?
            .error_for_status()?;
        Ok(response)
    }
}
