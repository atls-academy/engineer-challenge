use crate::application::commands::CommandHandler;
use crate::application::commands::auth::login::LoginCommand;
use crate::infrastructure::adapters::graphql::handlers::UserIp;
use crate::infrastructure::adapters::graphql::rate_limit::config::RateLimitRule;
use crate::infrastructure::adapters::graphql::rate_limit::limiter::RateLimiter;
use crate::infrastructure::adapters::http::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::LoginInput;
use crate::presentation::api::graphql::mutations::extract_cookie;
use async_graphql::{Context, Object, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<bool> {
        let limiter = ctx.data_unchecked::<RateLimiter>();
        let rules = ctx.data_unchecked::<HashMap<&str, RateLimitRule>>();
        let ip = ctx
            .data_opt::<UserIp>()
            .map(|u| u.0.as_str())
            .unwrap_or("unknown");

        if let Some(rule) = rules.get("login") {
            limiter
                .check("login", ip, rule)
                .await
                .map_err(|e| async_graphql::Error::new(e))?;
        }

        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let credentials = input
            .try_into()
            .map_err(|e: crate::domain::errors::DomainError| {
                async_graphql::Error::new(e.to_string())
            })?;
        let command = LoginCommand {
            credentials,
            cookie: extract_cookie(ctx),
        };
        let session_token = use_cases
            .commands
            .login
            .handle(command)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        if let Some(cookies) = ctx.data_opt::<ResponseCookies>() {
            cookies.add_cookie(session_token).await;
        }
        Ok(true)
    }
}
