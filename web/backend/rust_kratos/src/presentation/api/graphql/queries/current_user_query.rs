use crate::application::queries::QueryHandler;
use crate::application::queries::get_current_user::GetCurrentUserQuery;
use crate::infrastructure::adapters::graphql::handlers::UserIp;
use crate::infrastructure::adapters::graphql::rate_limit::config::RateLimitRule;
use crate::infrastructure::adapters::graphql::rate_limit::limiter::RateLimiter;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::UserProfileOutput;
use crate::presentation::api::graphql::queries::extract_cookie;
use async_graphql::{Context, Object, Result};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct CurrentUserQuery;

#[Object]
impl CurrentUserQuery {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<UserProfileOutput> {
        let limiter = ctx.data_unchecked::<RateLimiter>();
        let rules = ctx.data_unchecked::<HashMap<&str, RateLimitRule>>();
        let ip = ctx
            .data_opt::<UserIp>()
            .map(|u| u.0.as_str())
            .unwrap_or("unknown");

        if let Some(rule) = rules.get("currentUser") {
            limiter
                .check("currentUser", ip, rule)
                .await
                .map_err(|e| async_graphql::Error::new(e))?;
        }

        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();
        let query = GetCurrentUserQuery {
            cookie: extract_cookie(ctx),
        };
        use_cases
            .queries
            .get_current_user
            .handle(query)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
    }
}
