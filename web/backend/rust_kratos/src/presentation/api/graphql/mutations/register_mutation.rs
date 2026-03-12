use crate::application::commands::CommandHandler;
use crate::application::commands::identity::register::RegisterCommand;
use crate::infrastructure::adapters::graphql::handlers::UserIp;
use crate::infrastructure::adapters::graphql::rate_limit::config::RateLimitRule;
use crate::infrastructure::adapters::graphql::rate_limit::limiter::RateLimiter;
use crate::infrastructure::adapters::http::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::RegisterInput;
use async_graphql::{Context, Object, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct RegisterMutation;

#[Object]
impl RegisterMutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<bool> {
        let limiter = ctx.data_unchecked::<RateLimiter>();
        let rules = ctx.data_unchecked::<HashMap<&str, RateLimitRule>>();
        let ip = ctx
            .data_opt::<UserIp>()
            .map(|u| u.0.as_str())
            .unwrap_or("unknown");

        if let Some(rule) = rules.get("register") {
            limiter
                .check("register", ip, rule)
                .await
                .map_err(|e| async_graphql::Error::new(e))?;
        }

        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let command = RegisterCommand {
            data: input
                .try_into()
                .map_err(|e: crate::domain::errors::DomainError| {
                    async_graphql::Error::new(e.to_string())
                })?,
        };
        let result = use_cases
            .commands
            .register
            .handle(command)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        if let Some(cookies) = ctx.data_opt::<ResponseCookies>() {
            cookies.add_cookie(result.session_cookie).await;
        }
        Ok(true)
    }
}
