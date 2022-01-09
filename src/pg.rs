use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

pub async fn create_pg_pool(config: &crate::config::Config) -> crate::Result<PgPool> {
    Ok(PgPoolOptions::new()
        .max_connections(config.postgres.max_connections)
        .connect_timeout(Duration::from_secs(config.postgres.connect_timeout_secs))
        .connect(&config.postgres.url)
        .await?)
}
