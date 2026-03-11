use crate::application::commands::CommandHandler;
use crate::application::commands::account::settings::{
    UpdateSettingsCommand, UpdateSettingsResult,
};
use crate::domain::ports::settings::SettingsData;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::UpdateSettingsInput;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct SettingsMutation;

#[Object]
impl SettingsMutation {
    async fn update_settings(
        &self,
        ctx: &Context<'_>,
        input: UpdateSettingsInput,
    ) -> Result<String> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let command = UpdateSettingsCommand {
            data: SettingsData::from(input),
            cookie: extract_cookie(ctx).unwrap_or_default(),
        };

        let UpdateSettingsResult { flow_id, messages } = use_cases
            .commands
            .update_settings
            .handle(command)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            for message in messages {
                response_cookies.add_cookie(message).await;
            }
        }

        Ok(flow_id)
    }
}

fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
