use crate::application::commands::CommandHandler;
use crate::domain::errors::{AuthError, DomainError};
use crate::domain::ports::SessionPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct LogoutCommand {
    pub cookie: Option<String>,
}

pub struct LogoutCommandHandler {
    session_port: Arc<dyn SessionPort>,
}

impl LogoutCommandHandler {
    pub fn new(session_port: Arc<dyn SessionPort>) -> Self {
        Self { session_port }
    }
}

#[async_trait]
impl CommandHandler<LogoutCommand> for LogoutCommandHandler {
    async fn handle(&self, command: LogoutCommand) -> Result<(), DomainError> {
        let cookie = command.cookie.ok_or(AuthError::NotAuthenticated)?;
        self.session_port.logout(&cookie).await
    }
}
