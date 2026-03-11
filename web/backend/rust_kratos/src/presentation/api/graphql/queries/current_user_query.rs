use crate::application::queries::QueryHandler;
use crate::application::queries::get_current_user::GetCurrentUserQuery;
use crate::domain::entities::user_profile::UserProfile;
use crate::infrastructure::di::container::UseCases;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct CurrentUserQuery;

#[Object]
impl CurrentUserQuery {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<UserProfile> {
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

fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
