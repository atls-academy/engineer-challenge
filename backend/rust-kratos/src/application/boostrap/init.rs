use crate::application::boostrap::config::Config;
use crate::infrastructure::adapters::http::server;
use crate::infrastructure::di::container::AppContainer;
use crate::presentation::api::graphql::schema::{AppSchema, create_schema};
use std::sync::Arc;
use tokio::signal;
use tokio::signal::unix::SignalKind;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, reload};

pub async fn run() -> anyhow::Result<()> {
    let (filter, reload_handle) = reload::Layer::new(EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .map_err(|e| anyhow::anyhow!("Failed to initialize tracing subscriber: {}", e))?;

    let config = Config::from_env()?;

    reload_handle
        .modify(|f| *f = EnvFilter::new(&config.server.log_level))
        .map_err(|e| anyhow::anyhow!("Failed to reload log level: {}", e))?;

    info!("Starting application...");

    let container = AppContainer::new(&config)?;
    let schema = Arc::new(create_schema(&container));

    tokio::select! {
        result = start_server(schema, &config, &container) => result?,
        _ = shutdown_signal() => info!("Shutdown signal received, starting graceful shutdown..."),
    }

    Ok(())
}

async fn start_server(
    schema: Arc<AppSchema>,
    config: &Config,
    container: &AppContainer,
) -> anyhow::Result<()> {
    server::start(schema, config.server.clone(), container.kratos.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))
}

async fn shutdown_signal() {
    let mut sigint =
        signal::unix::signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");

    let mut sigterm =
        signal::unix::signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");

    let mut sigquit =
        signal::unix::signal(SignalKind::quit()).expect("Failed to install SIGQUIT handler");

    let mut sighup =
        signal::unix::signal(SignalKind::hangup()).expect("Failed to install SIGHUP handler");

    tokio::select! {
        _ = sigint.recv() => warn!(signal = "SIGINT", code = 2, "Received shutdown signal"),
        _ = sigterm.recv() => info!(signal = "SIGTERM", code = 15, "Received shutdown signal"),
        _ = sigquit.recv() => warn!(signal = "SIGQUIT", code = 3, "Received shutdown signal"),
        _ = sighup.recv() => warn!(signal = "SIGHUP", code = 1, "Received shutdown signal"),
    }
}
