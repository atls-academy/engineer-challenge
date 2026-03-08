use crate::application::usecases::auth::verification::VerificationUseCase;
use crate::presentation::api::graphql::inputs::{
    SendVerificationCodeInput, SubmitVerificationCodeInput, VerifyByLinkInput,
};
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct VerificationMutation;

#[Object]
impl VerificationMutation {
    async fn verify_by_link(&self, ctx: &Context<'_>, input: VerifyByLinkInput) -> Result<bool> {
        let verification_use_case = ctx.data_unchecked::<VerificationUseCase>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        verification_use_case
            .execute_link(input.into(), cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(true)
    }

    async fn send_verification_code(
        &self,
        ctx: &Context<'_>,
        input: SendVerificationCodeInput,
    ) -> Result<bool> {
        let verification_use_case = ctx.data_unchecked::<VerificationUseCase>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .map(|s| s.as_str());

        verification_use_case
            .execute_code_send(input.into(), cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(true)
    }

    async fn submit_verification_code(
        &self,
        ctx: &Context<'_>,
        input: SubmitVerificationCodeInput,
    ) -> Result<bool> {
        let verification_use_case = ctx.data_unchecked::<VerificationUseCase>();
        let cookie = ctx
            .data_opt::<Option<String>>()
            .and_then(|opt| opt.as_ref())
            .ok_or_else(|| {
                async_graphql::Error::new("Cookie is required to submit verification code")
            })?;

        verification_use_case
            .execute_code_submit(input.into(), cookie)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        Ok(true)
    }
}
