use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::settings::{SettingsData, SettingsPort};
use crate::domain::value_objects::auth_method::AuthMethod;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::models::errors::KratosFlowError;
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::debug;

pub struct KratosSettingsAdapter {
    client: Arc<KratosClient>,
}

impl KratosSettingsAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[derive(serde::Serialize)]
struct SettingsPayload {
    method: AuthMethod,
    csrf_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traits: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_secret_confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_secret_disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_secret_regenerate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_secret_reveal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transient_payload: Option<serde_json::Value>,
}

impl SettingsPayload {
    fn from_data(data: SettingsData, csrf_token: String) -> Self {
        let method = match data.method.as_str() {
            "password" => AuthMethod::Password,
            "code" => AuthMethod::Code,
            _ => AuthMethod::Link,
        };
        Self {
            method,
            csrf_token,
            password: data.password,
            traits: data.traits,
            lookup_secret_confirm: data.lookup_secret_confirm,
            lookup_secret_disable: data.lookup_secret_disable,
            lookup_secret_regenerate: data.lookup_secret_regenerate,
            lookup_secret_reveal: data.lookup_secret_reveal,
            transient_payload: data.transient_payload,
        }
    }
}

fn map_settings_error(e: KratosFlowError) -> DomainError {
    match (e.status, e.message_id()) {
        (StatusCode::FORBIDDEN, _) => AuthError::PrivilegedSessionRequired.into(),
        (StatusCode::UNAUTHORIZED, _) => AuthError::NotAuthenticated.into(),
        (StatusCode::GONE, _) => DomainError::NotFound("settings flow".into()),
        (StatusCode::BAD_REQUEST, 4000010) => {
            DomainError::InvalidData("Password is too weak".into())
        }
        (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().into()),
        _ => DomainError::ServiceUnavailable(e.to_string()),
    }
}

#[async_trait]
impl SettingsPort for KratosSettingsAdapter {
    async fn initiate_settings(&self, cookie: &str) -> Result<String, DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "settings",
            Some(cookie),
        )
        .await
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        Ok(flow.flow_id.as_str().to_string())
    }

    async fn update_settings(
        &self,
        _flow_id: &str,
        data: SettingsData,
        cookie: &str,
    ) -> Result<(String, Vec<String>), DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "settings",
            Some(cookie),
        )
        .await
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        let payload = SettingsPayload::from_data(data, flow.csrf_token.clone());

        debug!(
            "Settings payload: {}",
            serde_json::to_string_pretty(&payload).unwrap_or_default()
        );

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "settings",
            &flow.flow_id,
            serde_json::to_value(payload).map_err(|e| DomainError::InvalidData(e.to_string()))?,
            &flow.cookies,
        )
        .await
        .map_err(map_settings_error)?;

        debug!("Settings response: {:?}", result.data);
        debug!("Settings response cookies: {:?}", result.cookies);

        let state = result
            .data
            .get("state")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| DomainError::ServiceUnavailable("No state in response".into()))?;

        Ok((state, result.cookies))
    }
}
