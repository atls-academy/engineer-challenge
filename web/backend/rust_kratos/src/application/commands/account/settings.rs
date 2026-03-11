use crate::application::commands::CommandHandler;
use crate::domain::errors::DomainError;
use crate::domain::ports::{SettingsData, SettingsPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UpdateSettingsCommand {
    pub data: SettingsData,
    pub cookie: String,
}

pub struct UpdateSettingsResult {
    pub flow_id: String,
    pub messages: Vec<String>,
}

pub struct UpdateSettingsCommandHandler {
    settings_port: Arc<dyn SettingsPort>,
}

impl UpdateSettingsCommandHandler {
    pub fn new(settings_port: Arc<dyn SettingsPort>) -> Self {
        Self { settings_port }
    }
}

#[async_trait]
impl CommandHandler<UpdateSettingsCommand, UpdateSettingsResult> for UpdateSettingsCommandHandler {
    async fn handle(
        &self,
        command: UpdateSettingsCommand,
    ) -> Result<UpdateSettingsResult, DomainError> {
        let flow_id = self
            .settings_port
            .initiate_settings(&command.cookie)
            .await?;
        let (flow_id, messages) = self
            .settings_port
            .update_settings(&flow_id, command.data, &command.cookie)
            .await?;
        Ok(UpdateSettingsResult { flow_id, messages })
    }
}
