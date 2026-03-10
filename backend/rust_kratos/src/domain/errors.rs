#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Not authenticated")]
    NotAuthenticated,
    #[error("Session expired")]
    SessionExpired,
    #[error("Already logged in")]
    AlreadyLoggedIn,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Privileged session required")]
    PrivilegedSessionRequired,
}

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}
