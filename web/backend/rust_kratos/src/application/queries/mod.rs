use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait QueryHandler<Q, R>: Send + Sync {
    async fn handle(&self, query: Q) -> Result<R, DomainError>;
}

pub mod get_current_user;
pub mod health_check;
