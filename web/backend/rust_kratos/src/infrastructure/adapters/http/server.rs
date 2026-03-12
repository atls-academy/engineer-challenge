use crate::application::bootstrap::config::ServerConfig;
use crate::infrastructure::adapters::graphql::handlers::{graphql_handler, graphql_playground};
use crate::infrastructure::adapters::graphql::schema::AppSchema;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::presentation::api::rest::health_check;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use actix_web_prometheus::PrometheusMetricsBuilder;
use anyhow::Context;
use std::sync::Arc;
use tracing::info;
use tracing_actix_web::TracingLogger;

pub async fn start(
    schema: Arc<AppSchema>,
    config: ServerConfig,
    kratos_client: Arc<KratosClient>,
) -> anyhow::Result<()> {
    let bind_address = format!("{}:{}", config.host, config.port);
    info!("Booting HTTP server at http://{}", bind_address);

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build Prometheus metrics: {}", e))?;

    let cors_max_age = config.cors_max_age;
    let cors_allowed_origins = config.cors_allowed_origins.clone();
    let bind_address_clone = bind_address.clone();

    let server = HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
                http::header::ACCEPT,
                http::header::HeaderName::from_static("x-forwarded-for"),
            ])
            .supports_credentials()
            .max_age(cors_max_age);

        if cors_allowed_origins.contains(&"*".to_string()) {
            cors = cors.allow_any_origin();
        } else {
            cors = cors.supports_credentials();
            for origin in &cors_allowed_origins {
                cors = cors.allowed_origin(origin);
            }
        }

        App::new()
            .wrap(prometheus.clone())
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(web::Data::from(schema.clone()))
            .app_data(web::Data::new(kratos_client.clone()))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground)),
            )
            .configure(health_check::configure)
    })
    .bind(&bind_address_clone)
    .with_context(|| format!("Failed to bind server to {}", bind_address_clone))?;

    info!(
        "✅ HTTP server successfully started on http://{}",
        bind_address
    );
    info!("🚀 GraphQL Playground: http://{}/graphql", bind_address);
    info!("📊 Prometheus Metrics: http://{}/metrics", bind_address);

    server
        .run()
        .await
        .map_err(|e| anyhow::anyhow!("Server runtime error: {}", e))
}
