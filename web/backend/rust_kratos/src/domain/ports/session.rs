use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait SessionPort: Send + Sync {
    async fn logout(&self, cookie: &str) -> Result<(), DomainError>;
    async fn check_active_session(&self, cookie: Option<&str>) -> bool;
}
