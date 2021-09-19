use actix_web as aw;

#[aw::get("/start")]
async fn start(config: aw::web::Data<drama::config::Config>) -> aw::HttpResponse {
    let scope = "identity,edit,history,mysubreddits,read";
    let redirect_uri = "http://127.0.0.1:9999/";
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match drama::config::Config::from_env() {
        Ok(config) => config,
        Err(_) => {
            let err = std::io::Error::new(std::io::ErrorKind::Other, "could not read config");
            return Err(err);
        }
    };
    let factory = move || {
        aw::App::new()
            .app_data(aw::web::Data::new(config.clone()))
            .service(start)
    };
    aw::HttpServer::new(factory)
        .bind("127.0.0.1:9999")?
        .run()
        .await
}
