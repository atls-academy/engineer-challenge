use crate::application::queries::QueryHandler;
use crate::application::queries::health_check::{HealthCheckQuery, HealthCheckQueryHandler};
use actix_web::{HttpResponse, Responder, get, web};

#[get("/health")]
async fn health() -> impl Responder {
    let handler = HealthCheckQueryHandler;
    match handler.handle(HealthCheckQuery).await {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
