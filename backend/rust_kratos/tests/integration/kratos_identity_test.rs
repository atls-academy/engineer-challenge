use rust_kratos::domain::ports::identity::IdentityPort;
use rust_kratos::infrastructure::adapters::kratos::http::identity::KratosIdentityAdapter;
#[path = "../common/mod.rs"]
mod common;
use common::TestContext;

#[tokio::test]
async fn test_get_current_user_without_cookie_returns_error() {
    let ctx = TestContext::new();
    let adapter = KratosIdentityAdapter::new(ctx.client.clone());
    let result = adapter.get_current_user("invalid_cookie=abc").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_current_user_after_login_returns_profile() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let username = format!("user_{}", uuid::Uuid::new_v4());
    let session_cookie = register_and_login(&ctx, &email, &username, password).await;
    let adapter = KratosIdentityAdapter::new(ctx.client.clone());
    let result = adapter.get_current_user(&session_cookie).await;
    assert!(result.is_ok());
    let profile = result.unwrap();
    assert_eq!(profile.email, email);
    assert_eq!(profile.username, username);
}

#[tokio::test]
async fn test_get_current_user_returns_email_and_username() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let username = format!("user_{}", uuid::Uuid::new_v4());
    let session_cookie = register_and_login(&ctx, &email, &username, password).await;
    let adapter = KratosIdentityAdapter::new(ctx.client.clone());
    let profile = adapter.get_current_user(&session_cookie).await.unwrap();
    assert!(!profile.email.is_empty());
    assert!(!profile.username.is_empty());
}

async fn register_and_login(
    ctx: &TestContext,
    email: &str,
    username: &str,
    password: &str,
) -> String {
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
            email: email.to_string(),
            username: username.to_string(),
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
        identifier: email.to_string(),
        password: password.to_string(),
        address: None,
        code: None,
        resend: None,
    };
    adapter.complete_login(&flow_id, credentials).await.unwrap()
}
