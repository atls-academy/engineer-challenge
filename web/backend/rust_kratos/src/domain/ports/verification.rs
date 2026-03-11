use crate::domain::errors::DomainError;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct VerifyByLinkRequest {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct SendCodeRequest {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct SubmitCodeRequest {
    pub code: String,
    pub transient_payload: Option<Value>,
}

#[async_trait]
pub trait VerificationPort: Send + Sync {
    async fn verify_by_link(
        &self,
        request: VerifyByLinkRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError>;
    async fn send_verification_code(
        &self,
        request: SendCodeRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError>;
    async fn submit_verification_code(
        &self,
        request: SubmitCodeRequest,
        cookie: &str,
    ) -> Result<(), DomainError>;
}
