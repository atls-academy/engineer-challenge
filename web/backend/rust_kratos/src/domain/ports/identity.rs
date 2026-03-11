use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait IdentityPort: Send + Sync {
    async fn get_current_user(&self, cookie: &str) -> Result<UserProfile, DomainError>;
}
