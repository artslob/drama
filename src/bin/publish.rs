use std::time::Duration;

use drama::task::{Cron, Data, Task};
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Channel,
    Connection, ConnectionProperties,
};
use log::info;

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
    channel.basic_qos(1, BasicQosOptions::default()).await?;

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

    let schedule = vec![
        (Cron::CreateUserCron, 10),
        (Cron::UpdateUserSubredditsCron, 20),
        (Cron::UpdateUserInfoCron, 30),
    ];

    for (cron, secs) in schedule {
        let duration = Duration::from_secs(secs);
        let task = Task {
            common: Default::default(),
            data: Data::Cron(cron),
        };
        tokio::spawn(send_task(channel.clone(), task, duration));
    }

    loop {
        tokio::time::sleep(Duration::from_secs(60 * 5)).await;
    }
}

async fn send_task(channel: Channel, task: Task, duration: Duration) -> drama::Result<()> {
    // TODO do not use ? operator as it makes func to return
    loop {
        info!("sending task task {:?}", task);
        let properties = BasicProperties::default().with_delivery_mode(2);
        let confirm = channel
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                bincode::serialize(&task)?,
                properties,
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        tokio::time::sleep(duration).await
    }
}
