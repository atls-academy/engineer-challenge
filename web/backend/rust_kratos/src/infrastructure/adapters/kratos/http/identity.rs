use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::identity::IdentityPort;
use crate::infrastructure::adapters::kratos::client::KratosClient;
use async_trait::async_trait;
use reqwest::header;
use std::sync::Arc;

pub struct KratosIdentityAdapter {
    client: Arc<KratosClient>,
}

impl KratosIdentityAdapter {
    pub fn new(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

#[derive(serde::Deserialize)]
struct SessionResponse {
    identity: Identity,
}

#[derive(serde::Deserialize)]
struct Identity {
    traits: Traits,
}

#[derive(serde::Deserialize)]
struct Traits {
    email: String,
    username: String,
    geo_location: Option<String>,
}

#[async_trait]
impl IdentityPort for KratosIdentityAdapter {
    async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, DomainError> {
        let url =
            format!("{}/sessions/whoami", self.client.public_url).replace("localhost", "127.0.0.1");

        let response = self
            .client
            .client
            .get(&url)
            .header(header::COOKIE, cookie)
            .send()
            .await
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AuthError::NotAuthenticated.into());
        }

        let session: SessionResponse = response
            .json()
            .await
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        Ok(UserProfile::from(session.identity.traits))
    }
}

impl From<Traits> for UserProfile {
    fn from(t: Traits) -> Self {
        Self {
            email: t.email,
            username: t.username,
            geo_location: t.geo_location,
        }
    }
}
