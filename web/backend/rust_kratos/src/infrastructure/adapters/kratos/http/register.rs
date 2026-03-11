use crate::domain::errors::DomainError;
use crate::domain::ports::registration::{RegistrationData, RegistrationPort};
use crate::domain::value_objects::auth_method::AuthMethod;
use crate::domain::value_objects::session_cookie::SessionCookie;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::models::errors::KratosFlowError;
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct KratosRegistrationAdapter {
    client: Arc<KratosClient>,
}

impl KratosRegistrationAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[derive(serde::Serialize)]
struct RegistrationTraits {
    email: String,
    username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_location: Option<String>,
}

#[derive(serde::Serialize)]
struct RegistrationPayload {
    method: AuthMethod,
    password: String,
    traits: RegistrationTraits,
    csrf_token: String,
}

fn map_registration_error(e: KratosFlowError) -> DomainError {
    match (e.status, e.message_id()) {
        (StatusCode::BAD_REQUEST, 4000007) => DomainError::Conflict("Email already exists".into()),
        (StatusCode::BAD_REQUEST, 4000010) => {
            DomainError::InvalidData("Password is too weak".into())
        }
        (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().into()),
        (StatusCode::GONE, _) => DomainError::NotFound("registration flow".into()),
        _ => DomainError::ServiceUnavailable(e.to_string()),
    }
}

#[async_trait]
impl RegistrationPort for KratosRegistrationAdapter {
    async fn initiate_registration(&self, cookie: Option<&str>) -> Result<String, DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "registration",
            cookie,
        )
        .await
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        Ok(flow.flow_id.as_str().to_string())
    }

    async fn complete_registration(
        &self,
        _flow_id: &str,
        data: RegistrationData,
    ) -> Result<String, DomainError> {
        let flow = fetch_flow(
            &self.client.client,
            &self.client.public_url,
            "registration",
            None,
        )
        .await
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        let payload = RegistrationPayload {
            method: AuthMethod::Password,
            password: data.password,
            traits: RegistrationTraits {
                email: data.email,
                username: data.username,
                geo_location: data.geo_location,
            },
            csrf_token: flow.csrf_token.clone(),
        };

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "registration",
            &flow.flow_id,
            serde_json::to_value(payload).map_err(|e| DomainError::InvalidData(e.to_string()))?,
            &flow.cookies,
        )
        .await
        .map_err(map_registration_error)?;

        if result.data.get("session").is_none() && result.data.get("identity").is_none() {
            return Err(DomainError::ServiceUnavailable(
                "Neither session nor identity found in response".into(),
            ));
        }

        SessionCookie::find_in(result.cookies)
            .map(|c| c.as_str().to_string())
            .ok_or_else(|| DomainError::ServiceUnavailable("No session cookie was created".into()))
    }
}
