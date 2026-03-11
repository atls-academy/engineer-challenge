use crate::application::commands::CommandHandler;
use crate::application::commands::identity::register::RegisterCommand;
use crate::domain::ports::registration::RegistrationData;
use crate::infrastructure::adapters::graphql::cookies::ResponseCookies;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::RegisterInput;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct RegisterMutation;

#[Object]
impl RegisterMutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<bool> {
        let use_cases = ctx.data_unchecked::<Arc<UseCases>>();

        let command = RegisterCommand {
            data: RegistrationData::from(input),
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
