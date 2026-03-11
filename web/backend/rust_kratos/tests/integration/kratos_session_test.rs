use rust_kratos::domain::ports::session::SessionPort;
use rust_kratos::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
#[path = "../common/mod.rs"]
mod common;
use common::TestContext;

#[tokio::test]
async fn test_check_active_session_without_cookie_returns_false() {
    let ctx = TestContext::new();
    let adapter = KratosSessionAdapter::new(ctx.client.clone());
    let result = adapter.check_active_session(None).await;
    assert!(!result);
}

#[tokio::test]
async fn test_check_active_session_with_invalid_cookie_returns_false() {
    let ctx = TestContext::new();
    let adapter = KratosSessionAdapter::new(ctx.client.clone());
    let result = adapter.check_active_session(Some("invalid=abc")).await;
    assert!(!result);
}

#[tokio::test]
async fn test_logout_with_invalid_cookie_returns_error() {
    let ctx = TestContext::new();
    let adapter = KratosSessionAdapter::new(ctx.client.clone());
    let result = adapter.logout("invalid=abc").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logout_after_login_succeeds() {
    let ctx = TestContext::new();
    let session_cookie = register_and_login(&ctx).await;
    let adapter = KratosSessionAdapter::new(ctx.client.clone());
    let result = adapter.logout(&session_cookie).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_session_inactive_after_logout() {
    let ctx = TestContext::new();
    let session_cookie = register_and_login(&ctx).await;
    let adapter = KratosSessionAdapter::new(ctx.client.clone());
    adapter.logout(&session_cookie).await.unwrap();
    let result = adapter.check_active_session(Some(&session_cookie)).await;
    assert!(!result);
}

async fn register_and_login(ctx: &TestContext) -> String {
    use rust_kratos::domain::ports::login::{AuthenticationPort, LoginCredentials};
    use rust_kratos::domain::value_objects::auth_method::AuthMethod;
    use rust_kratos::infrastructure::adapters::kratos::http::flows::{fetch_flow, post_flow};
    use rust_kratos::infrastructure::adapters::kratos::http::login::KratosAuthenticationAdapter;

    #[derive(serde::Serialize)]
    struct RegistrationTraits {
        email: String,
        username: String,
    }

    #[derive(serde::Serialize)]
    struct RegistrationPayload {
        method: AuthMethod,
        password: String,
        traits: RegistrationTraits,
        csrf_token: String,
    }

    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let username = format!("user_{}", uuid::Uuid::new_v4());

    let flow = fetch_flow(
        &ctx.client.client,
        &ctx.client.public_url,
        "registration",
        None,
    )
    .await
    .unwrap();

    let payload = RegistrationPayload {
        method: AuthMethod::Password,
        password: password.to_string(),
        traits: RegistrationTraits {
            email: email.clone(),
            username,
        },
        csrf_token: flow.csrf_token.clone(),
    };

    post_flow(
        &ctx.client.client,
        &ctx.client.public_url,
        "registration",
        &flow.flow_id,
        serde_json::to_value(payload).unwrap(),
        &flow.cookies,
    )
    .await
    .unwrap();

    let adapter = KratosAuthenticationAdapter::new(ctx.client.clone());
    let flow_id = adapter.initiate_login(None).await.unwrap();
    let credentials = LoginCredentials {
        identifier: email,
        password: password.to_string(),
        address: None,
        code: None,
        resend: None,
    };
    adapter.complete_login(&flow_id, credentials).await.unwrap()
}
