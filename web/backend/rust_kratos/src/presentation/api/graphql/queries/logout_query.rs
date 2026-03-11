use crate::application::commands::CommandHandler;
use crate::application::commands::auth::logout::LogoutCommand;
use crate::infrastructure::di::container::UseCases;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct LogoutQuery;

#[Object]
impl LogoutQuery {
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let command = LogoutCommand {
            cookie: extract_cookie(ctx),
        };

        use_cases
            .commands
            .logout
            .handle(command)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(true)
    }
}

fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
