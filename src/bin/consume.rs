use futures_util::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use log::info;
use uuid::Uuid;

use drama::task::Task;

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
        tokio::spawn(handle_task(channel.clone(), task));
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
    Ok(())
}

async fn handle_task(channel: Channel, task: Task) -> drama::Result<()> {
    info!("msg waited");
    match task {
        Task::CreateUser { common, uid } => {
            info!(
                "got task to create user created at {} with row uuid {}",
                common.created_at, uid
            );
        }
        Task::CreateUserCron(_) => create_user_cron(channel).await?,
        _ => {}
    };
    Ok(())
}

async fn create_user_cron(channel: Channel) -> drama::Result<()> {
    // TODO select tokens and send them as personal tasks
    info!("got cron task to create user... sending new task");
    channel
        .basic_publish(
            "",
            "hello",
            BasicPublishOptions::default(),
            bincode::serialize(&Task::CreateUser {
                common: Default::default(),
                uid: Uuid::new_v4(),
            })?,
            BasicProperties::default().with_delivery_mode(2),
        )
        .await?
        .await?;
    Ok(())
}
