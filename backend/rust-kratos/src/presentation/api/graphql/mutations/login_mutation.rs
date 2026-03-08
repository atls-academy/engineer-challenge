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
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        let session_token = use_cases
            .login
            .execute(LoginCredentials::from(input), cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if let Some(response_cookies) = ctx.data_opt::<ResponseCookies>() {
            response_cookies.add_cookie(session_token).await;
        }

        Ok(true)
    }
}
