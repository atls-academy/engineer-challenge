use crate::domain::ports::recovery::RecoveryRequest;
use crate::infrastructure::di::container::UseCases;
use crate::presentation::api::graphql::inputs::RecoveryInput;
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use tracing::info;

#[derive(Default)]
pub struct RecoveryMutation;

#[Object]
impl RecoveryMutation {
    async fn recovery(&self, ctx: &Context<'_>, input: RecoveryInput) -> Result<bool> {
        info!("Recovery mutation called");
        let use_cases = ctx
            .data::<Arc<UseCases>>()
            .map_err(|e| async_graphql::Error::new(format!("DI error: {:?}", e)))?;

        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        use_cases
            .recovery
            .execute(RecoveryRequest::from(input), cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(true)
    }
}
