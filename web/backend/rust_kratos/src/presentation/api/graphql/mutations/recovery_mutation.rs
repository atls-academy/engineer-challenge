use crate::application::commands::CommandHandler;
use crate::application::commands::account::recovery::RecoveryCommand;
use crate::domain::ports::recovery::RecoveryRequest;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::RecoveryInput;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

#[derive(Default)]
pub struct RecoveryMutation;

#[Object]
impl RecoveryMutation {
    async fn recovery(&self, ctx: &Context<'_>, input: RecoveryInput) -> Result<bool> {
        let use_cases = ctx
            .data::<Arc<UseCases>>()
            .map_err(|e| async_graphql::Error::new(format!("DI error: {:?}", e)))?;

        let command = RecoveryCommand {
            request: RecoveryRequest::from(input),
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

fn extract_cookie(ctx: &Context<'_>) -> Option<String> {
    ctx.data_opt::<Option<String>>().and_then(|opt| opt.clone())
}
