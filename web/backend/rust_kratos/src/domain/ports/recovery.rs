use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RecoveryRequest {
    pub email: String,
}

#[async_trait]
pub trait RecoveryPort: Send + Sync {
    async fn initiate_recovery(
        &self,
        request: RecoveryRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError>;
}
