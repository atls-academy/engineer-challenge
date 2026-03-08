use crate::domain::errors::DomainError;
use crate::domain::ports::login::{AuthenticationPort, LoginCredentials};
use crate::domain::ports::session::SessionPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use async_trait::async_trait;
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::{debug, error};

pub struct KratosAuthenticationAdapter {
    client: Arc<KratosClient>,
    session_adapter: KratosSessionAdapter,
}

impl KratosAuthenticationAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        let session_adapter = KratosSessionAdapter::new(client.clone());
        Self {
            client,
            session_adapter,
        }
    }
}

#[async_trait]
impl AuthenticationPort for KratosAuthenticationAdapter {
    async fn initiate_login(&self, cookie: Option<&str>) -> Result<String, DomainError> {
        if self.session_adapter.check_active_session(cookie).await
            && !self.session_adapter.is_recovery_session(cookie).await {
                error!("Login attempt with an already active session");
                return Err(DomainError::AlreadyLoggedIn);
            }

        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        flow.flow["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(DomainError::FlowNotFound)
    }

    async fn complete_login(
        &self,
        flow_id: &str,
        credentials: LoginCredentials,
    ) -> Result<String, DomainError> {
        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| DomainError::Network(e.to_string()))?;

        let csrf_token = flow.csrf_token.clone();
        debug!("Using flow_id: {}, csrf_token: {}", flow_id, csrf_token);

        let mut payload = serde_json::json!({
            "method": "password",
            "identifier": credentials.identifier,
            "password": credentials.password,
            "csrf_token": csrf_token,
        });

        if let Some(addr) = credentials.address {
            payload["address"] = serde_json::json!(addr);
        }
        if let Some(code) = credentials.code {
            payload["code"] = serde_json::json!(code);
            payload["method"] = serde_json::json!("code");
        }
        if let Some(resend) = credentials.resend {
            payload["resend"] = serde_json::json!(resend);
        }

        debug!(
            "Login payload: {}",
            serde_json::to_string_pretty(&payload).unwrap()
        );

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "login",
            flow_id,
            payload,
            &flow.cookies,
        )
        .await
        .map_err(|e| {
            error!("Failed to post login flow: {}", e);
            match (e.status, e.message_id()) {
                (StatusCode::BAD_REQUEST, 4000006) => DomainError::InvalidCredentials,
                (StatusCode::BAD_REQUEST, 4000010) => DomainError::InvalidCredentials,
                (StatusCode::BAD_REQUEST, _) => {
                    DomainError::InvalidData(e.message_text().to_string())
                }
                (StatusCode::GONE, _) => DomainError::FlowNotFound,
                (StatusCode::UNAUTHORIZED, _) => DomainError::NotAuthenticated,
                _ => DomainError::Network(e.to_string()),
            }
        })?;

        debug!("Received cookies: {:?}", result.cookies);
        debug!("Response data: {:?}", result.data);

        if result.cookies.is_empty() {
            error!("No cookies in response");
            return Err(DomainError::Unknown(
                "No cookies received from server".to_string(),
            ));
        }

        result
            .cookies
            .into_iter()
            .find(|c| c.contains("session") || c.starts_with("ory_"))
            .ok_or_else(|| {
                error!("Session cookie not found in response cookies");
                DomainError::Unknown("Session token not found".to_string())
            })
    }
}
