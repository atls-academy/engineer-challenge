use rust_kratos::domain::ports::recovery::{RecoveryPort, RecoveryRequest};
use rust_kratos::infrastructure::adapters::kratos::http::recovery::KratosRecoveryAdapter;

#[path = "../common/mod.rs"]
mod common;
use common::TestContext;

#[tokio::test]
async fn test_initiate_recovery_with_nonexistent_email_succeeds() {
    let ctx = TestContext::new();
    let adapter = KratosRecoveryAdapter::new(ctx.client.clone());

    let result = adapter
        .initiate_recovery(
            RecoveryRequest {
                email: "nonexistent@example.com".to_string(),
            },
            None,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_initiate_recovery_with_existing_email_succeeds() {
    let ctx = TestContext::new();
    let email = register_user(&ctx).await;
    let adapter = KratosRecoveryAdapter::new(ctx.client.clone());

    let result = adapter
        .initiate_recovery(RecoveryRequest { email }, None)
        .await;

    assert!(result.is_ok());
}

async fn register_user(ctx: &TestContext) -> String {
    use rust_kratos::domain::ports::registration::{RegistrationData, RegistrationPort};
    use rust_kratos::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;

    let email = TestContext::random_email();
    let adapter = KratosRegistrationAdapter::new(ctx.client.clone());
    let flow_id = adapter.initiate_registration(None).await.unwrap();

    adapter
        .complete_registration(
            &flow_id,
            RegistrationData {
                email: email.clone(),
                username: format!("user_{}", uuid::Uuid::new_v4()),
                password: "Test1234!@#$".to_string(),
                geo_location: None,
            },
        )
        .await
        .unwrap();

    email
}
