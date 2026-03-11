use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::login::{AuthenticationPort, LoginCredentials};
use crate::domain::ports::session::SessionPort;
use crate::domain::value_objects::auth_method::AuthMethod;
use crate::domain::value_objects::session_cookie::SessionCookie;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use crate::infrastructure::adapters::kratos::models::errors::KratosFlowError;
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

#[derive(serde::Serialize)]
struct LoginPayload {
    method: AuthMethod,
    identifier: String,
    password: String,
    csrf_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resend: Option<String>,
}

impl LoginPayload {
    fn from_credentials(credentials: LoginCredentials, csrf_token: String) -> Self {
        Self {
            method: if credentials.code.is_some() {
                AuthMethod::Code
            } else {
                AuthMethod::Password
            },
            identifier: credentials.identifier,
            password: credentials.password,
            csrf_token,
            address: credentials.address,
            code: credentials.code,
            resend: credentials.resend,
        }
    }
}

fn map_login_error(e: KratosFlowError) -> DomainError {
    error!("Failed to post login flow: {}", e);
    match (e.status, e.message_id()) {
        (StatusCode::BAD_REQUEST, 4000006 | 4000010) => AuthError::InvalidCredentials.into(),
        (StatusCode::BAD_REQUEST, _) => DomainError::InvalidData(e.message_text().into()),
        (StatusCode::GONE, _) => DomainError::NotFound("login flow".into()),
        (StatusCode::UNAUTHORIZED, _) => AuthError::NotAuthenticated.into(),
        _ => DomainError::ServiceUnavailable(e.to_string()),
    }
}

#[async_trait]
impl AuthenticationPort for KratosAuthenticationAdapter {
    async fn initiate_login(&self, cookie: Option<&str>) -> Result<String, DomainError> {
        let is_active = self.session_adapter.check_active_session(cookie).await;
        let is_recovery = self.session_adapter.is_recovery_session(cookie).await;

        if is_active && !is_recovery {
            error!("Login attempt with an already active session");
            return Err(AuthError::AlreadyLoggedIn.into());
        }

        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        Ok(flow.flow_id.as_str().to_string())
    }

    async fn complete_login(
        &self,
        _flow_id: &str,
        credentials: LoginCredentials,
    ) -> Result<String, DomainError> {
        let flow = fetch_flow(&self.client.client, &self.client.public_url, "login", None)
            .await
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        let payload = LoginPayload::from_credentials(credentials, flow.csrf_token.clone());

        debug!(
            "Login payload: {}",
            serde_json::to_string_pretty(&payload).unwrap_or_default()
        );

        let result = post_flow(
            &self.client.client,
            &self.client.public_url,
            "login",
            &flow.flow_id,
            serde_json::to_value(payload).map_err(|e| DomainError::InvalidData(e.to_string()))?,
            &flow.cookies,
        )
        .await
        .map_err(map_login_error)?;

        debug!("Received cookies: {:?}", result.cookies);
        debug!("Response data: {:?}", result.data);

        SessionCookie::find_in(result.cookies)
            .map(|c| c.as_str().to_string())
            .ok_or_else(|| {
                error!("Session cookie not found in response cookies");
                DomainError::ServiceUnavailable("Session token not found".into())
            })
    }
}
