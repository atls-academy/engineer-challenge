use crate::application::commands::CommandHandler;
use crate::application::commands::auth::login::LoginCommand;
use crate::domain::ports::login::LoginCredentials;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::LoginInput;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let command = LoginCommand {
            credentials: LoginCredentials::from(input),
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

fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
