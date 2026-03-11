use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::verification::{
    SendCodeRequest, SubmitCodeRequest, VerificationPort, VerifyByLinkRequest,
};
use crate::domain::value_objects::auth_method::AuthMethod;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::models::errors::KratosFlowError;
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;

pub struct KratosVerificationAdapter {
    client: Arc<KratosClient>,
}

impl KratosVerificationAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[derive(serde::Serialize)]
struct VerificationPayload {
    method: AuthMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    csrf_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    transient_payload: Option<serde_json::Value>,
}

fn map_verification_error(e: KratosFlowError) -> DomainError {
    match (e.status, e.message_id()) {
        (StatusCode::BAD_REQUEST, 4070006) => {
            DomainError::InvalidData("Invalid verification code".into())
        }
        (StatusCode::BAD_REQUEST, 4070001) => {
            DomainError::InvalidData("Invalid email address".into())
        }
        (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().into()),
        (StatusCode::GONE, _) => DomainError::NotFound("verification flow".into()),
        (StatusCode::UNAUTHORIZED, _) => AuthError::NotAuthenticated.into(),
        _ => DomainError::ServiceUnavailable(e.to_string()),
    }
}

async fn execute_verification_flow(
    client: &KratosClient,
    method: AuthMethod,
    email: Option<String>,
    code: Option<String>,
    transient_payload: Option<serde_json::Value>,
    cookie: Option<&str>,
) -> Result<(), DomainError> {
    let flow = fetch_flow(&client.client, &client.public_url, "verification", cookie)
        .await
        .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

    let payload = VerificationPayload {
        method,
        email,
        code,
        csrf_token: flow.csrf_token.clone(),
        transient_payload,
    };

    post_flow(
        &client.client,
        &client.public_url,
        "verification",
        &flow.flow_id,
        serde_json::to_value(payload).map_err(|e| DomainError::InvalidData(e.to_string()))?,
        &flow.cookies,
    )
    .await
    .map_err(map_verification_error)?;

    Ok(())
}

#[async_trait]
impl VerificationPort for KratosVerificationAdapter {
    async fn verify_by_link(
        &self,
        request: VerifyByLinkRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        execute_verification_flow(
            &self.client,
            AuthMethod::Link,
            Some(request.email),
            None,
            request.transient_payload,
            cookie,
        )
        .await
    }

    async fn send_verification_code(
        &self,
        request: SendCodeRequest,
        cookie: Option<&str>,
    ) -> Result<(), DomainError> {
        execute_verification_flow(
            &self.client,
            AuthMethod::Code,
            Some(request.email),
            None,
            request.transient_payload,
            cookie,
        )
        .await
    }

    async fn submit_verification_code(
        &self,
        request: SubmitCodeRequest,
        cookie: &str,
    ) -> Result<(), DomainError> {
        execute_verification_flow(
            &self.client,
            AuthMethod::Code,
            None,
            Some(request.code),
            request.transient_payload,
            Some(cookie),
        )
        .await
    }
}
