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
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(container.use_cases.clone())
    .finish()
}
