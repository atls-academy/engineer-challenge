use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct SettingsData {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<serde_json::Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<serde_json::Value>,
}

#[async_trait]
pub trait SettingsPort: Send + Sync {
    async fn initiate_settings(&self, cookie: &str) -> Result<String, DomainError>;
    async fn update_settings(
        &self,
        flow_id: &str,
        data: SettingsData,
        cookie: &str,
    ) -> Result<(String, Vec<String>), DomainError>;
}
