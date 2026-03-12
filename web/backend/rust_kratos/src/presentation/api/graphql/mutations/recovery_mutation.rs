use crate::application::commands::CommandHandler;
use crate::application::commands::account::recovery::RecoveryCommand;
use crate::infrastructure::adapters::graphql::handlers::UserIp;
use crate::infrastructure::adapters::graphql::rate_limit::config::RateLimitRule;
use crate::infrastructure::adapters::graphql::rate_limit::limiter::RateLimiter;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::RecoveryInput;
use crate::presentation::api::graphql::mutations::extract_cookie;
use async_graphql::{Context, Object, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct RecoveryMutation;

#[Object]
impl RecoveryMutation {
    async fn recovery(&self, ctx: &Context<'_>, input: RecoveryInput) -> Result<bool> {
        let limiter = ctx.data_unchecked::<RateLimiter>();
        let rules = ctx.data_unchecked::<HashMap<&str, RateLimitRule>>();
        let ip = ctx
            .data_opt::<UserIp>()
            .map(|u| u.0.as_str())
            .unwrap_or("unknown");

        if let Some(rule) = rules.get("recovery") {
            limiter
                .check("recovery", ip, rule)
                .await
                .map_err(|e| async_graphql::Error::new(e))?;
        }

        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let command = RecoveryCommand {
            request: input
                .try_into()
                .map_err(|e: crate::domain::errors::DomainError| {
                    async_graphql::Error::new(e.to_string())
                })?,
            cookie: extract_cookie(ctx),
        };
        use_cases
            .commands
            .recovery
            .handle(command)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}
