use crate::application::commands::CommandHandler;
use crate::domain::errors::DomainError;
use crate::domain::ports::{
    SendCodeRequest, SubmitCodeRequest, VerificationPort, VerifyByLinkRequest,
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct VerifyByLinkCommand {
    pub request: VerifyByLinkRequest,
    pub cookie: Option<String>,
}

pub struct SendVerificationCodeCommand {
    pub request: SendCodeRequest,
    pub cookie: Option<String>,
}

pub struct SubmitVerificationCodeCommand {
    pub request: SubmitCodeRequest,
    pub cookie: String,
}

pub struct VerificationCommandHandler {
    verification_port: Arc<dyn VerificationPort>,
}

impl VerificationCommandHandler {
    pub fn new(verification_port: Arc<dyn VerificationPort>) -> Self {
        Self { verification_port }
    }
}

#[async_trait]
impl CommandHandler<VerifyByLinkCommand> for VerificationCommandHandler {
    async fn handle(&self, command: VerifyByLinkCommand) -> Result<(), DomainError> {
        self.verification_port
            .verify_by_link(command.request, command.cookie.as_deref())
            .await
    }
}

#[async_trait]
impl CommandHandler<SendVerificationCodeCommand> for VerificationCommandHandler {
    async fn handle(&self, command: SendVerificationCodeCommand) -> Result<(), DomainError> {
        self.verification_port
            .send_verification_code(command.request, command.cookie.as_deref())
            .await
    }
}

#[async_trait]
impl CommandHandler<SubmitVerificationCodeCommand> for VerificationCommandHandler {
    async fn handle(&self, command: SubmitVerificationCodeCommand) -> Result<(), DomainError> {
        self.verification_port
            .submit_verification_code(command.request, &command.cookie)
            .await
    }
}
