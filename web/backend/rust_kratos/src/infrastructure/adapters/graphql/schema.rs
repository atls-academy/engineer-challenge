use crate::infrastructure::adapters::graphql::rate_limit::config::rules_from_config;
use crate::infrastructure::adapters::graphql::rate_limit::limiter::RateLimiter;
use crate::infrastructure::di::container::AppContainer;
use crate::presentation::api::graphql::mutations::recovery_mutation::RecoveryMutation;
use crate::presentation::api::graphql::mutations::register_mutation::RegisterMutation;
use crate::presentation::api::graphql::mutations::settings_mutation::SettingsMutation;
use crate::presentation::api::graphql::mutations::verify_mutation::VerificationMutation;
use crate::presentation::api::graphql::queries::current_user_query::CurrentUserQuery;
use crate::presentation::api::graphql::{
    mutations::login_mutation::LoginMutation, queries::logout_query::LogoutQuery,
};
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct QueryRoot(CurrentUserQuery, LogoutQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    RegisterMutation,
    LoginMutation,
    RecoveryMutation,
    SettingsMutation,
    VerificationMutation,
);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(container: &AppContainer) -> AppSchema {
    let rate_limiter = RateLimiter::new(container.cache.clone());
    let rules = rules_from_config(&container.rate_limits);

    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(container.use_cases.clone())
    .data(rate_limiter)
    .data(rules)
    .finish()
}
