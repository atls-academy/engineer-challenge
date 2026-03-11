use crate::application::queries::QueryHandler;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

pub struct HealthCheckQuery;

pub struct HealthCheckQueryHandler;

#[async_trait]
impl QueryHandler<HealthCheckQuery, String> for HealthCheckQueryHandler {
    async fn handle(&self, _query: HealthCheckQuery) -> Result<String, DomainError> {
        Ok("OK".to_string())
    }
}
