use futures_util::StreamExt;
use lapin::{options::{BasicPublishOptions, QueueDeclareOptions}, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties};
use tokio_amqp::LapinTokioExt;
use anyhow::Result;
use tracing::{instrument};
#[derive(Clone)]
pub struct RabbitMQPublisher {
  channel: Channel,
  exchange: String,
}

impl RabbitMQPublisher {
  pub async fn new(uri: &str, exchange: &str) -> Result<Self> {
    let connection = Connection::connect(uri, ConnectionProperties::default().with_tokio()).await?;
    let channel = connection.create_channel().await?;

    channel
      .queue_bind(
          "event_rust",
          "exchange_direct",
          "event_rust",
          lapin::options::QueueBindOptions::default(),
          FieldTable::default(),
      )
      .await?;

    channel
        .queue_declare(
            "event_nest",
            QueueDeclareOptions {
                durable: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    Ok(Self {
      channel,
      exchange: exchange.to_string(),
    })
  }

  #[instrument(skip(self, routing_key, payload))]
  pub async fn publish(&self, routing_key: &str, payload: &[u8]) -> Result<()> {
    self.channel
      .basic_publish(
        &self.exchange,
        routing_key,
        BasicPublishOptions::default(),
        payload,
        BasicProperties::default(),
      )
      .await?
      .await?; 
    Ok(())
  }

  #[instrument(skip(self, queue, handler))]
  pub async fn consume<F, Fut>(&self, queue: &str, handler: F) -> Result<()>
  where
      F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
      Fut: std::future::Future<Output = Result<()>> + Send,
  {
      let mut consumer = self
          .channel
          .basic_consume(
              queue,
              "",
              lapin::options::BasicConsumeOptions::default(),
              lapin::types::FieldTable::default(),
          )
          .await?;
  
      while let Some(delivery) = consumer.next().await {
          if let Ok(delivery) = delivery {
              let data = delivery.data.clone();
              if let Err(err) = handler(data).await {
                  eprintln!("Error handling message: {:?}", err);
              }
              delivery
                  .ack(lapin::options::BasicAckOptions::default())
                  .await
                  .unwrap_or_else(|e| eprintln!("Failed to ack message: {}", e));
          }
      }
  
      Ok(())
  }
}