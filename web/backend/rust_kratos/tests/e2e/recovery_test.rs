use reqwest::Client;
use rust_kratos::domain::ports::recovery::{RecoveryPort, RecoveryRequest};
use rust_kratos::domain::ports::registration::{RegistrationData, RegistrationPort};
use rust_kratos::infrastructure::adapters::kratos::http::recovery::KratosRecoveryAdapter;
use rust_kratos::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;

#[path = "../common/mod.rs"]
mod common;
use common::{MailhogClient, TestContext};

async fn register_user(ctx: &TestContext) -> String {
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

#[tokio::test]
async fn test_recovery_email_is_sent_for_existing_user() {
    let ctx = TestContext::new();
    let mailhog = MailhogClient::new();
    let email = register_user(&ctx).await;

    mailhog.delete_all().await;

    let adapter = KratosRecoveryAdapter::new(ctx.client.clone());
    let result = adapter
        .initiate_recovery(
            RecoveryRequest {
                email: email.clone(),
            },
            None,
        )
        .await;

    assert!(result.is_ok());

    let link = mailhog.fetch_recovery_link(&email).await;
    assert!(link.is_some(), "Recovery email not received in MailHog");
    assert!(link.unwrap().contains("recovery"));
}

#[tokio::test]
async fn test_recovery_link_is_valid_and_redirects() {
    let ctx = TestContext::new();
    let mailhog = MailhogClient::new();
    let email = register_user(&ctx).await;

    mailhog.delete_all().await;

    let adapter = KratosRecoveryAdapter::new(ctx.client.clone());
    adapter
        .initiate_recovery(
            RecoveryRequest {
                email: email.clone(),
            },
            None,
        )
        .await
        .unwrap();

    let link = mailhog
        .fetch_recovery_link(&email)
        .await
        .expect("Recovery link not found in MailHog");

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let resp = client.get(&link).send().await.unwrap();

    assert!(
        resp.status().is_success() || resp.status().is_redirection(),
        "Recovery link returned unexpected status: {}",
        resp.status()
    );
}
