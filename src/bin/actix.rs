use actix_web as aw;
use drama::model::RegistrationToken;
use drama::reddit::model::Token;
use reqwest::Client as HttpClient;

#[aw::get("/")]
async fn start(config: aw::web::Data<drama::config::Config>) -> aw::HttpResponse {
    let scope = "identity,history,mysubreddits,read";
    let redirect_uri = "http://127.0.0.1:9999/callback";
    let state = "64990aeb-5178-43d3-8ccb-110962843622";
    let url = format!(
        "https://www.reddit.com/api/v1/authorize?\
        client_id={client_id}\
        &response_type=code&state={state}\
        &redirect_uri={redirect_uri}\
        &duration=permanent\
        &scope={scope}",
        client_id = config.client_id,
        state = state,
        redirect_uri = redirect_uri,
        scope = scope,
    );
    let link = format!(r#"<a href="{url}">go here</a>"#, url = url);
    aw::HttpResponse::build(aw::http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(link)
}

#[derive(Debug, serde::Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

async fn insert_token(pool: &sqlx::PgPool, token: Token) -> Result<RegistrationToken, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let token: RegistrationToken = sqlx::query_as::<_, RegistrationToken>(
        "INSERT INTO registration_token (uuid, access_token, refresh_token, token_type, \
    expires_in, scope) VALUES ($1, $2, $3, $4, $5, $6)  \
    RETURNING uuid, access_token, refresh_token, token_type, expires_in, scope",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(token.access_token)
    .bind(token.refresh_token)
    .bind(token.token_type)
    .bind(token.expires_in)
    .bind(token.scope)
    .fetch_one(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(token)
}

async fn request_and_insert_token(
    config: aw::web::Data<drama::config::Config>,
    pool: aw::web::Data<sqlx::PgPool>,
    body: String,
) -> drama::Result<RegistrationToken> {
    let token: Token = HttpClient::new()
        .post(&config.access_token_url)
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .body(body)
        .send()
        .await
        .map_err(|e| drama::Error::from(e.to_string()))?
        .error_for_status()
        .map_err(|e| drama::Error::from(e.to_string()))?
        .json()
        .await
        .map_err(|e| drama::Error::from(e.to_string()))?;
    let token = insert_token(&pool, token).await?;
    Ok(token)
}

#[aw::get("/callback")]
async fn callback(
    params: aw::web::Query<CallbackParams>,
    config: aw::web::Data<drama::config::Config>,
    pool: aw::web::Data<sqlx::PgPool>,
) -> Result<impl aw::Responder, aw::Error> {
    println!("code {}\nstate {}", params.code, params.state);
    let redirect_uri = "http://127.0.0.1:9999/callback";
    let body = format!(
        "grant_type=authorization_code&code={code}&redirect_uri={redirect_uri}",
        code = params.code,
        redirect_uri = redirect_uri
    );
    let token = match request_and_insert_token(config, pool, body).await {
        Ok(token) => token,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    };
    println!("{:#?}", token);
    Ok("nice")
}

#[actix_web::main]
async fn main() -> drama::Result<()> {
    let config = drama::config::Config::from_env()?;

    let pool = drama::pg::create_pg_pool(&config).await?;

    let factory = move || {
        aw::App::new()
            .app_data(aw::web::Data::new(config.clone()))
            .app_data(aw::web::Data::new(pool.clone()))
            .service(start)
            .service(callback)
    };
    aw::HttpServer::new(factory)
        .bind("127.0.0.1:9999")?
        .run()
        .await
        .map_err(Into::into)
}
