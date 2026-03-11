use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct RegistrationData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub geo_location: Option<String>,
}

#[async_trait]
pub trait RegistrationPort: Send + Sync {
    async fn initiate_registration(&self, cookie: Option<&str>) -> Result<String, DomainError>;
    async fn complete_registration(
        &self,
        flow_id: &str,
        data: RegistrationData,
    ) -> Result<String, DomainError>;
}
