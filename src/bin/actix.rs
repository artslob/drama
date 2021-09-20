use actix_web as aw;

#[aw::get("/start")]
async fn start(config: aw::web::Data<drama::config::Config>) -> aw::HttpResponse {
    let scope = "identity,edit,history,mysubreddits,read";
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

#[aw::get("/callback")]
async fn callback(params: aw::web::Query<CallbackParams>) -> impl aw::Responder {
    println!("code {}\nstate {}", params.code, params.state);
    "nice"
}

#[actix_web::main]
async fn main() -> drama::Result<()> {
    let config = drama::config::Config::from_env()?;
    let factory = move || {
        aw::App::new()
            .app_data(aw::web::Data::new(config.clone()))
            .service(start)
            .service(callback)
    };
    aw::HttpServer::new(factory)
        .bind("127.0.0.1:9999")?
        .run()
        .await
        .map_err(Into::into)
}
