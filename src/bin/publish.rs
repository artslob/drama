use std::time::Duration;

use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
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
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Declared queue {:?}", queue);

    loop {
        let now = chrono::Local::now().to_string();
        let payload = format!("message constructed at {}", now);
        let confirm = channel
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload.clone().into_bytes(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        println!("sent {}", payload);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
