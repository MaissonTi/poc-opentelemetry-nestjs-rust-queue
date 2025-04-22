mod db;
mod consumer;
mod queue;
mod lib;

use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use opentelemetry_sdk::{trace, Resource, runtime};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     dotenv().ok();
//     init_tracer();

//     let database_url = env::var("DATABASE_URL")?;
//     let rabbitmq_url = env::var("RABBITMQ_URL")?;

//     let pool = PgPool::connect(&database_url).await?;

//     tracing::info!("Connected to PostgreSQL");

//     // Inicia o consumidor em uma tarefa separada
//     let consumer_handle = tokio::spawn(async move {
//         if let Err(err) = consumer::start_consumer(pool, &rabbitmq_url).await {
//             tracing::error!("Consumer error: {:?}", err);
//         }
//     });

//     // Aguarda até que o programa receba um sinal de término (Ctrl+C)
//     tokio::signal::ctrl_c().await?;
//     tracing::info!("Shutting down application...");

//     // Aguarda o término do consumidor
//     consumer_handle.await?;

//     // Finaliza o tracer do OpenTelemetry
//     opentelemetry::global::shutdown_tracer_provider();

//     Ok(())
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracer()?;

    let database_url = env::var("DATABASE_URL")?;
    let rabbitmq_url = env::var("RABBITMQ_URL")?;

    let pool = PgPool::connect(&database_url).await?;

    consumer::start_consumer(pool, &rabbitmq_url).await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

fn init_tracer() -> anyhow::Result<()> {

    use opentelemetry::global;
    use opentelemetry_sdk::propagation::TraceContextPropagator;

    // ✅ REGISTRA O PROPAGADOR CORRETO
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317");

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace::config().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "rust-tracing"),
        ])))
        .install_batch(runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())

}