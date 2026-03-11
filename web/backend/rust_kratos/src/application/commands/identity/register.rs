use crate::application::commands::CommandHandler;
use crate::domain::errors::DomainError;
use crate::domain::ports::{RegistrationData, RegistrationPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct RegisterCommand {
    pub data: RegistrationData,
}

pub struct RegisterCommandResult {
    pub flow_id: String,
    pub session_cookie: String,
}

pub struct RegisterCommandHandler {
    registration_port: Arc<dyn RegistrationPort>,
}

impl RegisterCommandHandler {
    pub fn new(registration_port: Arc<dyn RegistrationPort>) -> Self {
        Self { registration_port }
    }
}

#[async_trait]
impl CommandHandler<RegisterCommand, RegisterCommandResult> for RegisterCommandHandler {
    async fn handle(&self, command: RegisterCommand) -> Result<RegisterCommandResult, DomainError> {
        let flow_id = self.registration_port.initiate_registration(None).await?;
        let session_cookie = self
            .registration_port
            .complete_registration(&flow_id, command.data)
            .await?;
        Ok(RegisterCommandResult {
            flow_id,
            session_cookie,
        })
    }
}
