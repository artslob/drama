use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Result};
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
        match std::str::from_utf8(&delivery.data) {
            Ok(s) => {
                let s = s.to_string();
                tokio::spawn(async move {
                    let secs = s.matches('#').count();
                    tokio::time::sleep(std::time::Duration::from_secs(secs as u64)).await;
                    info!("msg waited {}: {}", secs, s);
                });
            }
            Err(e) => {
                info!("error! {}", e)
            }
        }
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
    Ok(())
}
