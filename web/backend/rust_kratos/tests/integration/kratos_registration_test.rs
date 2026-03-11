use rust_kratos::domain::ports::registration::{RegistrationData, RegistrationPort};
use rust_kratos::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;

#[path = "../common/mod.rs"]
mod common;
use common::TestContext;

#[tokio::test]
async fn test_initiate_registration_returns_flow_id() {
    let ctx = TestContext::new();
    let adapter = KratosRegistrationAdapter::new(ctx.client.clone());

    let result = adapter.initiate_registration(None).await;

    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
async fn test_complete_registration_returns_session_cookie() {
    let ctx = TestContext::new();
    let adapter = KratosRegistrationAdapter::new(ctx.client.clone());
    let flow_id = adapter.initiate_registration(None).await.unwrap();

    let data = RegistrationData {
        email: TestContext::random_email(),
        username: format!("user_{}", uuid::Uuid::new_v4()),
        password: "Test1234!@#$".to_string(),
        geo_location: None,
    };

    let result = adapter.complete_registration(&flow_id, data).await;

    assert!(result.is_ok());
    assert!(result.unwrap().contains("ory_kratos_session"));
}

#[tokio::test]
async fn test_complete_registration_with_weak_password_fails() {
    let ctx = TestContext::new();
    let adapter = KratosRegistrationAdapter::new(ctx.client.clone());
    let flow_id = adapter.initiate_registration(None).await.unwrap();

    let data = RegistrationData {
        email: TestContext::random_email(),
        username: format!("user_{}", uuid::Uuid::new_v4()),
        password: "123".to_string(),
        geo_location: None,
    };

    let result = adapter.complete_registration(&flow_id, data).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_complete_registration_with_duplicate_email_fails() {
    let ctx = TestContext::new();
    let adapter = KratosRegistrationAdapter::new(ctx.client.clone());
    let email = TestContext::random_email();

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

    let flow_id = adapter.initiate_registration(None).await.unwrap();
    let result = adapter
        .complete_registration(
            &flow_id,
            RegistrationData {
                email,
                username: format!("user_{}", uuid::Uuid::new_v4()),
                password: "Test1234!@#$".to_string(),
                geo_location: None,
            },
        )
        .await;

    assert!(result.is_err());
}
