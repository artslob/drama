use reqwest::blocking::Client as HttpClient;
use reqwest::blocking::Response;

#[derive(serde::Deserialize, Debug)]
struct Token {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

// TODO get oauth and access_token urls from config
pub struct Reddit {
    client: HttpClient,
    token: Token,
    authorization: String,
    api: String,
    user_agent: String,
}

impl Reddit {
    pub fn new(config: &crate::config::Config) -> crate::Result<Self> {
        let client = HttpClient::new();
        let user_agent = config.user_agent.to_string();

        let token_url = "https://www.reddit.com/api/v1/access_token";
        let response = client
            .post(token_url)
            .header("user-agent", &user_agent)
            .basic_auth(&config.client_id, Some(&config.client_secret))
            .body("grant_type=client_credentials")
            .send()?
            .error_for_status()?;

        let api = "https://oauth.reddit.com/".into();
        let token: Token = response.json()?;
        let authorization = format!("bearer {}", &token.access_token);

        Ok(Reddit {
            client,
            token,
            authorization,
            api,
            user_agent,
        })
    }

    pub fn get(&self, tail: &str) -> crate::Result<Response> {
        let url = format!("{}/{}", self.api, tail);
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
