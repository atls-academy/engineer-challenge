use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler<C, R = ()>: Send + Sync {
    async fn handle(&self, command: C) -> Result<R, DomainError>;
}

pub mod account;
pub mod auth;
pub mod identity;
