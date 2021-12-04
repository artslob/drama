use drama::config::{Config, ConfigRef};
use drama::queue::Queue;
use drama::reddit::model::User;
use drama::task::{Cron, Data, Task, TaskCommon};
use futures::TryStreamExt;
use futures_util::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use log::{error, info};
use reqwest::Client;
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
            Queue::Default.name(),
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
            Queue::Default.name(),
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("will consume");
    while let Some(delivery) = consumer.next().await {
        let (channel, delivery) = delivery.expect("error in consumer");
        let task: Task = match bincode::deserialize(&delivery.data) {
            Ok(task) => task,
            Err(_) => continue,
        };
        tokio::spawn(handle_task(config, channel, task, pool.clone()));
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
    let task_name: &'static str = (&task.data).into();
    let common = task.common;
    let result = match task.data {
        Data::Cron(cron) => match cron {
            Cron::CreateUserCron => create_user_cron(channel, &pool).await,
            Cron::UpdateUserSubredditsCron => update_user_subreddits_cron(channel, &pool).await,
            Cron::UpdateUserInfoCron => update_user_info_cron(channel, &pool).await,
        },
        Data::CreateUser { uid } => create_user(config, &pool, common, uid).await,
        Data::UpdateUserSubreddits { user_id } => {
            update_user_subreddits(config, &pool, user_id).await
        }
        Data::UpdateUserInfo { user_id } => update_user_info(config, &pool, user_id).await,
    };
    match result {
        Ok(_) => info!("task {} handled successfully", task_name),
        Err(err) => error!("task {} was failed: {}", task_name, err),
    }
    Ok(())
}

async fn update_user_info(
    config: ConfigRef,
    pool: &sqlx::PgPool,
    user_id: String,
) -> drama::Result<()> {
    let access_token = get_actual_token(config, pool, &user_id).await?;

    let user: User = reqwest::Client::builder()
        .user_agent(config.user_agent.to_string())
        .build()?
        .get("https://oauth.reddit.com/api/v1/me")
        .bearer_auth(&access_token.access_token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"UPDATE "user" SET accept_followers = $1, has_subscribed = $2, has_verified_email = $3,
        hide_from_robots = $4, is_employee = $5, is_gold = $6, is_mod = $7, name = $8,
        total_karma = $9, link_karma = $10, awardee_karma = $11, awarder_karma = $12,
        comment_karma = $13, verified = $14
        WHERE id = $15"#,
    )
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
    .bind(user.id)
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(())
}

async fn update_user_info_cron(channel: Channel, pool: &sqlx::PgPool) -> drama::Result<()> {
    let mut ids = sqlx::query(r#"SELECT id FROM "user" LIMIT 10"#).fetch(pool);

    while let Some(row) = ids.try_next().await? {
        let task = Task {
            common: Default::default(),
            data: Data::UpdateUserInfo {
                user_id: row.try_get("id")?,
            },
        };
        channel
            .basic_publish(
                "",
                Queue::Default.name(),
                BasicPublishOptions::default(),
                bincode::serialize(&task)?,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?
            .await?;
    }

    Ok(())
}

#[derive(Debug, sqlx::FromRow)]
struct AccessToken {
    uuid: Uuid,
    user_id: String,
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

#[derive(serde::Deserialize, Debug, sqlx::FromRow)]
struct RefreshToken {
    // TODO created_at: String,
    // TODO updated_at: String,
    uuid: uuid::Uuid,
    user_id: String,
    refresh_token: String,
    token_type: String,
    scope: String,
}

#[derive(serde::Deserialize, Debug, sqlx::FromRow)]
struct Token {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
}

async fn refresh_token<'a>(
    config: ConfigRef,
    pool: &'a sqlx::PgPool,
    user_id: &'a str,
) -> drama::Result<AccessToken> {
    let refresh_token = sqlx::query_as::<_, RefreshToken>(
        r#"
        SELECT * FROM refresh_token
        WHERE user_id = $1 AND created_at + interval '1 year' > current_timestamp
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(&user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| drama::Error::from(format!("refresh token is expired for user {}", user_id)))?;

    let body = format!(
        "grant_type=refresh_token&refresh_token={}",
        refresh_token.refresh_token
    );
    let token: Token = Client::new()
        .post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(&config.client_id, Some(&config.client_secret))
        .body(body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    info!(
        "refresh token didnt change: {}",
        token.refresh_token == refresh_token.refresh_token
    );

    let access_token = AccessToken {
        uuid: Uuid::new_v4(),
        user_id: user_id.to_owned(),
        access_token: token.access_token,
        token_type: token.token_type,
        expires_in: token.expires_in,
        scope: token.scope,
    };

    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO access_token (uuid, user_id, access_token, token_type, expires_in, scope)
          VALUES ($1, $2, $3, $4, $5, $6)"#,
    )
    .bind(&access_token.uuid)
    .bind(&access_token.user_id)
    .bind(&access_token.access_token)
    .bind(&access_token.token_type)
    .bind(&access_token.expires_in)
    .bind(&access_token.scope)
    .execute(&mut tx)
    .await?;
    tx.commit().await?;

    Ok(access_token)
}

async fn get_actual_token<'a>(
    config: ConfigRef,
    pool: &'a sqlx::PgPool,
    user_id: &'a str,
) -> drama::Result<AccessToken> {
    // TODO extract 5 minutes from interval
    let access_token = sqlx::query_as::<_, AccessToken>(
        r#"
        SELECT * FROM access_token
        WHERE user_id = $1 AND created_at + expires_in * interval '1 second' > current_timestamp
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let access_token = match access_token {
        Some(token) => token,
        None => refresh_token(config, pool, user_id).await?,
    };
    Ok(access_token)
}

async fn update_user_subreddits(
    config: ConfigRef,
    pool: &sqlx::PgPool,
    user_id: String,
) -> drama::Result<()> {
    let access_token = get_actual_token(config, pool, &user_id).await?;

    use drama::reddit::model::{Data, Listing, Subreddit};

    // TODO process "before" items in pagination result
    let subreddits: Data<Listing<Data<Subreddit>>> = reqwest::Client::builder()
        .user_agent(config.user_agent.to_string())
        .build()?
        .get("https://oauth.reddit.com/subreddits/mine/subscriber")
        .bearer_auth(&access_token.access_token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    info!("{}", serde_json::to_string_pretty(&subreddits)?);

    let mut tx = pool.begin().await?;

    sqlx::query(r#"DELETE FROM subreddit WHERE user_id = $1"#)
        .bind(&user_id)
        .execute(&mut tx)
        .await?;

    for subreddit in subreddits.data.children.iter().map(|data| &data.data) {
        sqlx::query(
            r#"INSERT INTO subreddit
        (id, user_id, display_name, header_title, name,
        public_description, subreddit_type, subscribers, title, url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (id) DO NOTHING"#,
        )
        .bind(&subreddit.id)
        .bind(&user_id)
        .bind(&subreddit.display_name)
        .bind(&subreddit.header_title)
        .bind(&subreddit.name)
        .bind(&subreddit.public_description)
        .bind(&subreddit.subreddit_type)
        .bind(&subreddit.subscribers)
        .bind(&subreddit.title)
        .bind(&subreddit.url)
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

async fn update_user_subreddits_cron(channel: Channel, pool: &sqlx::PgPool) -> drama::Result<()> {
    let mut ids = sqlx::query(r#"SELECT id FROM "user" LIMIT 10"#).fetch(pool);

    while let Some(row) = ids.try_next().await? {
        let user_id: String = row.try_get("id")?;
        let task = Task {
            common: Default::default(),
            data: Data::UpdateUserSubreddits { user_id },
        };
        channel
            .basic_publish(
                "",
                Queue::Default.name(),
                BasicPublishOptions::default(),
                bincode::serialize(&task)?,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?
            .await?;
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
        let uid: Uuid = row.try_get("uuid")?;
        let task = Task {
            common: Default::default(),
            data: Data::CreateUser { uid },
        };
        channel
            .basic_publish(
                "",
                Queue::Default.name(),
                BasicPublishOptions::default(),
                bincode::serialize(&task)?,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await?
            .await?;
    }

    Ok(())
}
