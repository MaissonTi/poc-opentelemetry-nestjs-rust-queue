use crate::db::{insert_user, User};
use crate::lib::trace_utils::{HeaderExtractor, HeaderInjector};
use crate::queue::rabbitmq::RabbitMQPublisher;
use anyhow::Result;
use opentelemetry::global;
use opentelemetry::trace::TraceContextExt;
use opentelemetry::Context;
use serde_json::{json, Value};
use sqlx::PgPool;
use tracing::{info, instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;
use std::sync::Arc;

#[instrument(skip(pool, amqp_url))]
pub async fn start_consumer(pool: PgPool, amqp_url: &str) -> Result<()> {
    let rabbitmq = Arc::new(RabbitMQPublisher::new(amqp_url, "exchange_direct").await?);
    let pool = Arc::new(pool);

    info!("📥 Waiting for messages...");

    consumer_message(&Arc::clone(&rabbitmq), &pool).await?;

    Ok(())
}

#[instrument(skip(pool, rabbitmq))]
async fn consumer_message(rabbitmq: &Arc<RabbitMQPublisher>, pool: &PgPool) -> Result<()> {

    let pool = pool.clone();
    let rabbitmq = rabbitmq.clone();

    rabbitmq
    .consume("event_nest", {
        let pool = Arc::new(pool);
        let rabbitmq = Arc::clone(&rabbitmq);
        move |data| {
            let pool = Arc::clone(&pool);
            let rabbitmq = Arc::clone(&rabbitmq);
            async move {
                let (parent_cx, parsed) = extract_trace_context(&data)?;               
                
                save_message(parent_cx.clone(), &parsed, &pool).await?;

                let user = User {
                    id: Uuid::new_v4().to_string(),
                    name: "maisson_consumer_rust".to_string(),
                    email: format!("{}@example.com", Uuid::new_v4()),
                };

                publish_message(parent_cx.clone(), &rabbitmq, &user).await?;
                Ok(())
            }
        }
    })
    .await?;

    Ok(())
}

#[instrument(skip(parent_cx, rabbitmq, user))]
async fn publish_message(parent_cx: Context, rabbitmq: &RabbitMQPublisher, user: &User) -> Result<()> {
    tracing::Span::current().set_parent(parent_cx.clone());    

    let mut trace_context = std::collections::HashMap::new();
    global::get_text_map_propagator(|prop| {
        prop.inject_context(&parent_cx, &mut HeaderInjector(&mut trace_context));
    });

    let message = json!({
        "pattern": "event_rust",
        "data": user,
        "traceContext": trace_context,
    });

    let payload = serde_json::to_vec(&message)?;
    rabbitmq.publish("event_rust", &payload).await?;

    let trace_id = parent_cx.span().span_context().trace_id();
    info!("\u{1f4e4} Published message with trace_id: {:?}", trace_id);

    Ok(())
}

#[instrument(skip(pool, parsed))]
async fn save_message(parent_cx: Context, parsed: &Value, pool: &PgPool) -> Result<()> {
    tracing::Span::current().set_parent(parent_cx.clone());
    
    info!(?parsed, "\u{1f426} Consumer message");
    
    insert_user(pool).await?;
    Ok(())
}

fn extract_trace_context(data: &[u8]) -> Result<(Context, Value)> {
    let json = std::str::from_utf8(data)?;
    let parsed: Value = serde_json::from_str(json)?;

    let trace_context_obj = parsed
        .get("data")
        .and_then(|d| d.get("traceContext"))
        .and_then(|v| v.as_object());

    let trace_context_value = match trace_context_obj {
        Some(map) => map
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect(),
        None => std::collections::HashMap::new(),
    };

    let parent_cx = global::get_text_map_propagator(|prop| {
        prop.extract(&HeaderExtractor::new(&trace_context_value))
    });


    Ok((parent_cx, parsed))
}
