use crate::application::commands::CommandHandler;
use crate::application::commands::account::settings::{
    UpdateSettingsCommand, UpdateSettingsResult,
};
use crate::infrastructure::adapters::graphql::handlers::UserIp;
use crate::infrastructure::adapters::graphql::rate_limit::config::RateLimitRule;
use crate::infrastructure::adapters::graphql::rate_limit::limiter::RateLimiter;
use crate::infrastructure::adapters::http::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::UpdateSettingsInput;
use crate::presentation::api::graphql::mutations::extract_cookie;
use async_graphql::{Context, Object, Result};
use std::collections::HashMap;
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
        let limiter = ctx.data_unchecked::<RateLimiter>();
        let rules = ctx.data_unchecked::<HashMap<&str, RateLimitRule>>();
        let ip = ctx
            .data_opt::<UserIp>()
            .map(|u| u.0.as_str())
            .unwrap_or("unknown");

        if let Some(rule) = rules.get("settings") {
            limiter
                .check("settings", ip, rule)
                .await
                .map_err(|e| async_graphql::Error::new(e))?;
        }

        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let command = UpdateSettingsCommand {
            data: input
                .try_into()
                .map_err(|e: crate::domain::errors::DomainError| {
                    async_graphql::Error::new(e.to_string())
                })?,
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
