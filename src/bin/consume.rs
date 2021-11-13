use drama::config::{Config, ConfigRef};
use drama::reddit::model::User;
use drama::task::{Task, TaskCommon};
use futures::TryStreamExt;
use futures_util::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use log::info;
use sqlx::Row;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> drama::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    let conn = Connection::connect(
        &addr,
        ConnectionProperties::default().with_default_executor(8),
    )
    .await?;

    let config = Config::from_env()?.permanent();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect("postgres://drama_user:drama_pass@localhost:5932/drama_db")
        .await?;

    let channel = conn.create_channel().await?;

    let queue = channel
        .queue_declare(
            "hello",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    info!("Declared queue {:?}", queue);

    let mut consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("will consume");
    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.expect("error in consumer");
        let task = bincode::deserialize(&delivery.data);
        let task: Task = match task {
            Ok(task) => task,
            Err(_) => continue,
        };
        tokio::spawn(handle_task(config, channel.clone(), task, pool.clone()));
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
    Ok(())
}

async fn handle_task(
    config: ConfigRef,
    channel: Channel,
    task: Task,
    pool: sqlx::PgPool,
) -> drama::Result<()> {
    info!("msg waited");
    let result = match task {
        Task::CreateUser { common, uid } => create_user(config, &pool, common, uid).await,
        Task::CreateUserCron(_) => create_user_cron(channel, &pool).await,
        _ => return Ok(()),
    };
    match result {
        Ok(_) => info!("task handled successfully"),
        Err(err) => info!("result of handled task: {}", err),
    }
    Ok(())
}

async fn create_user(
    config: ConfigRef,
    pool: &sqlx::PgPool,
    common: TaskCommon,
    uid: Uuid,
) -> drama::Result<()> {
    info!(
        "got task to create user created at {} with row uuid {}",
        common.created_at, uid
    );
    let token =
        sqlx::query_as::<_, RegistrationToken>("SELECT * FROM registration_token WHERE uuid = $1")
            .bind(&uid)
            .fetch_optional(pool)
            .await?;
    let token = match token {
        Some(token) => token,
        None => {
            info!("not found registration token with uid {}", uid);
            return Ok(());
        }
    };
    let user: User = reqwest::Client::builder()
        .user_agent(config.user_agent.to_string())
        .build()?
        .get("https://oauth.reddit.com/api/v1/me")
        .bearer_auth(&token.access_token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    info!("got user with id {}", &user.id);
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO "user" (id, accept_followers, has_subscribed, has_verified_email,
        hide_from_robots, is_employee, is_gold, is_mod, name,
        total_karma, link_karma, awardee_karma, awarder_karma, comment_karma, verified)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        ON CONFLICT (id) DO NOTHING"#,
    )
    .bind(&user.id)
    .bind(user.accept_followers)
    .bind(user.has_subscribed)
    .bind(user.has_verified_email)
    .bind(user.hide_from_robots)
    .bind(user.is_employee)
    .bind(user.is_gold)
    .bind(user.is_mod)
    .bind(user.name)
    .bind(user.total_karma)
    .bind(user.link_karma)
    .bind(user.awardee_karma)
    .bind(user.awarder_karma)
    .bind(user.comment_karma)
    .bind(user.verified)
    .execute(&mut tx)
    .await?;
    sqlx::query(r#"DELETE FROM registration_token WHERE uuid = $1"#)
        .bind(&uid)
        .execute(&mut tx)
        .await?;
    sqlx::query(
        r"INSERT INTO refresh_token (uuid, user_id, refresh_token, token_type, scope)
          VALUES ($1, $2, $3, $4, $5)
          ON CONFLICT ON CONSTRAINT uq_refresh_token_refresh_token DO NOTHING",
    )
    .bind(Uuid::new_v4())
    .bind(&user.id)
    .bind(&token.refresh_token)
    .bind(&token.token_type)
    .bind(&token.scope)
    .execute(&mut tx)
    .await?;
    sqlx::query(
        r"INSERT INTO access_token (uuid, user_id, access_token, token_type, expires_in, scope)
          VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(Uuid::new_v4())
    .bind(&user.id)
    .bind(&token.access_token)
    .bind(&token.token_type)
    .bind(&token.expires_in)
    .bind(&token.scope)
    .execute(&mut tx)
    .await?;
    tx.commit().await?;
    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct RegistrationToken {
    uuid: Uuid,
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

async fn create_user_cron(channel: Channel, pool: &sqlx::PgPool) -> drama::Result<()> {
    info!("got cron task to create user... sending new tasks");

    let mut uuids = sqlx::query("SELECT uuid FROM registration_token LIMIT 10").fetch(pool);

    while let Some(row) = uuids.try_next().await? {
        let uuid: Uuid = row.try_get("uuid")?;
        channel
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                bincode::serialize(&Task::CreateUser {
                    common: Default::default(),
                    uid: uuid,
                })?,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?
            .await?;
    }

    Ok(())
}
