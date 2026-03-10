use rust_kratos::application::commands::CommandHandler;
use rust_kratos::application::commands::account::recovery::{
    RecoveryCommand, RecoveryCommandHandler,
};
use rust_kratos::application::commands::account::settings::{
    UpdateSettingsCommand, UpdateSettingsCommandHandler,
};
use rust_kratos::application::commands::auth::login::{LoginCommand, LoginCommandHandler};
use rust_kratos::application::commands::auth::logout::{LogoutCommand, LogoutCommandHandler};
use rust_kratos::application::commands::identity::register::{
    RegisterCommand, RegisterCommandHandler,
};
use rust_kratos::application::queries::QueryHandler;
use rust_kratos::application::queries::get_current_user::{
    GetCurrentUserQuery, GetCurrentUserQueryHandler,
};
use rust_kratos::domain::ports::login::LoginCredentials;
use rust_kratos::domain::ports::recovery::RecoveryRequest;
use rust_kratos::domain::ports::registration::RegistrationData;
use rust_kratos::domain::ports::settings::SettingsData;
use rust_kratos::infrastructure::adapters::kratos::http::{
    identity::KratosIdentityAdapter, login::KratosAuthenticationAdapter,
    logout::KratosSessionAdapter, recovery::KratosRecoveryAdapter,
    register::KratosRegistrationAdapter, settings::KratosSettingsAdapter,
};

#[path = "../common/mod.rs"]
mod common;
use common::TestContext;

fn make_register_handler(ctx: &TestContext) -> RegisterCommandHandler {
    RegisterCommandHandler::new(std::sync::Arc::new(KratosRegistrationAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_login_handler(ctx: &TestContext) -> LoginCommandHandler {
    LoginCommandHandler::new(std::sync::Arc::new(KratosAuthenticationAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_logout_handler(ctx: &TestContext) -> LogoutCommandHandler {
    LogoutCommandHandler::new(std::sync::Arc::new(KratosSessionAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_get_current_user_handler(ctx: &TestContext) -> GetCurrentUserQueryHandler {
    GetCurrentUserQueryHandler::new(std::sync::Arc::new(KratosIdentityAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_recovery_handler(ctx: &TestContext) -> RecoveryCommandHandler {
    RecoveryCommandHandler::new(std::sync::Arc::new(KratosRecoveryAdapter::new(
        ctx.client.clone(),
    )))
}

fn make_settings_handler(ctx: &TestContext) -> UpdateSettingsCommandHandler {
    UpdateSettingsCommandHandler::new(std::sync::Arc::new(KratosSettingsAdapter::new(
        ctx.client.clone(),
    )))
}

async fn register_and_login(ctx: &TestContext, email: &str, password: &str) -> String {
    let handler = make_register_handler(ctx);
    let result = handler
        .handle(RegisterCommand {
            data: RegistrationData {
                email: email.to_string(),
                password: password.to_string(),
                username: format!("user_{}", uuid::Uuid::new_v4()),
                geo_location: None,
            },
        })
        .await
        .unwrap();
    result.session_cookie
}

#[tokio::test]
async fn test_register_command_returns_session_cookie() {
    let ctx = TestContext::new();
    let handler = make_register_handler(&ctx);
    let result = handler
        .handle(RegisterCommand {
            data: RegistrationData {
                email: TestContext::random_email(),
                password: "Test1234!@#$".to_string(),
                username: format!("user_{}", uuid::Uuid::new_v4()),
                geo_location: None,
            },
        })
        .await;
    assert!(result.is_ok());
    let r = result.unwrap();
    assert!(!r.session_cookie.is_empty());
    assert!(!r.flow_id.is_empty());
}

#[tokio::test]
async fn test_login_command_returns_session_cookie() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    register_and_login(&ctx, &email, password).await;
    let handler = make_login_handler(&ctx);
    let result = handler
        .handle(LoginCommand {
            credentials: LoginCredentials {
                identifier: email.clone(),
                password: password.to_string(),
                address: None,
                code: None,
                resend: None,
            },
            cookie: None,
        })
        .await;
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[tokio::test]
async fn test_login_command_with_invalid_credentials_fails() {
    let ctx = TestContext::new();
    let handler = make_login_handler(&ctx);
    let result = handler
        .handle(LoginCommand {
            credentials: LoginCredentials {
                identifier: "nonexistent@example.com".to_string(),
                password: "wrongpassword".to_string(),
                address: None,
                code: None,
                resend: None,
            },
            cookie: None,
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logout_command_without_cookie_fails() {
    let ctx = TestContext::new();
    let handler = make_logout_handler(&ctx);
    let result = handler.handle(LogoutCommand { cookie: None }).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_logout_command_with_valid_session() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let cookie = register_and_login(&ctx, &email, password).await;
    let handler = make_logout_handler(&ctx);
    let result = handler
        .handle(LogoutCommand {
            cookie: Some(cookie),
        })
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_user_without_cookie_fails() {
    let ctx = TestContext::new();
    let handler = make_get_current_user_handler(&ctx);
    let result = handler.handle(GetCurrentUserQuery { cookie: None }).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_current_user_with_valid_session() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let cookie = register_and_login(&ctx, &email, password).await;
    let handler = make_get_current_user_handler(&ctx);
    let result = handler
        .handle(GetCurrentUserQuery {
            cookie: Some(cookie),
        })
        .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().email, email);
}

#[tokio::test]
async fn test_recovery_command_with_valid_email() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    register_and_login(&ctx, &email, password).await;
    let handler = make_recovery_handler(&ctx);
    let result = handler
        .handle(RecoveryCommand {
            request: RecoveryRequest { email },
            cookie: None,
        })
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_recovery_command_with_unknown_email() {
    let ctx = TestContext::new();
    let handler = make_recovery_handler(&ctx);
    let result = handler
        .handle(RecoveryCommand {
            request: RecoveryRequest {
                email: "ghost@example.com".to_string(),
            },
            cookie: None,
        })
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_settings_command_without_session_fails() {
    let ctx = TestContext::new();
    let handler = make_settings_handler(&ctx);
    let result = handler
        .handle(UpdateSettingsCommand {
            data: SettingsData {
                method: "password".to_string(),
                password: Some("NewPass1234!".to_string()),
                traits: None,
                lookup_secret_confirm: None,
                lookup_secret_disable: None,
                lookup_secret_regenerate: None,
                lookup_secret_reveal: None,
                transient_payload: None,
            },
            cookie: "invalid_cookie".to_string(),
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_settings_command_with_valid_session() {
    let ctx = TestContext::new();
    let email = TestContext::random_email();
    let password = "Test1234!@#$";
    let cookie = register_and_login(&ctx, &email, password).await;
    let handler = make_settings_handler(&ctx);
    let result = handler
        .handle(UpdateSettingsCommand {
            data: SettingsData {
                method: "password".to_string(),
                password: Some("NewPass5678!@#$".to_string()),
                traits: None,
                lookup_secret_confirm: None,
                lookup_secret_disable: None,
                lookup_secret_regenerate: None,
                lookup_secret_reveal: None,
                transient_payload: None,
            },
            cookie,
        })
        .await;
    assert!(result.is_err());
}
