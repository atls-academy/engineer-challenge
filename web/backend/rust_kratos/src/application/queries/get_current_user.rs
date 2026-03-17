use crate::application::queries::QueryHandler;
use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::outbound::identity::IdentityPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetCurrentUserQuery {
    pub cookie: Option<String>,
}

pub struct GetCurrentUserQueryHandler {
    identity_port: Arc<dyn IdentityPort>,
}

impl GetCurrentUserQueryHandler {
    pub fn new(identity_port: Arc<dyn IdentityPort>) -> Self {
        Self { identity_port }
    }
}

#[async_trait]
impl QueryHandler<GetCurrentUserQuery, UserProfile> for GetCurrentUserQueryHandler {
    async fn handle(&self, query: GetCurrentUserQuery) -> Result<UserProfile, DomainError> {
        let cookie = query.cookie.ok_or(AuthError::NotAuthenticated)?;
        self.identity_port.get_current_user(&cookie).await
    }
}
