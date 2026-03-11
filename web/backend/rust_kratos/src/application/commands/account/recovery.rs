use crate::application::commands::CommandHandler;
use crate::domain::errors::DomainError;
use crate::domain::ports::{RecoveryPort, RecoveryRequest};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{error, info};

pub struct RecoveryCommand {
    pub request: RecoveryRequest,
    pub cookie: Option<String>,
}

pub struct RecoveryCommandHandler {
    recovery_port: Arc<dyn RecoveryPort>,
}

impl RecoveryCommandHandler {
    pub fn new(recovery_port: Arc<dyn RecoveryPort>) -> Self {
        Self { recovery_port }
    }
}

#[async_trait]
impl CommandHandler<RecoveryCommand> for RecoveryCommandHandler {
    async fn handle(&self, command: RecoveryCommand) -> Result<(), DomainError> {
        info!(
            email = &command.request.email,
            cookie_present = command.cookie.is_some(),
            "Starting recovery process"
        );
        self.recovery_port
            .initiate_recovery(command.request, command.cookie.as_deref())
            .await
            .map_err(|e| {
                error!(error = %e, "Recovery failed");
                e
            })?;
        info!("Recovery email sent successfully");
        Ok(())
    }
}
