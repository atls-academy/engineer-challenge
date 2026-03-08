use async_graphql::InputObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::ports::login::LoginCredentials;
use crate::domain::ports::recovery::RecoveryRequest;
use crate::domain::ports::registration::RegistrationData;
use crate::domain::ports::settings::SettingsData;
use crate::domain::ports::verification::{SendCodeRequest, SubmitCodeRequest, VerifyByLinkRequest};

#[derive(InputObject, Clone)]
pub struct RegisterInput {
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct LoginInput {
    pub identifier: String,
    pub password: String,
    pub address: Option<String>,
    pub code: Option<String>,
    pub resend: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug, Clone)]
pub struct RecoveryInput {
    pub email: String,
}

#[derive(InputObject)]
pub struct VerifyByLinkInput {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject)]
pub struct SendVerificationCodeInput {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject)]
pub struct SubmitVerificationCodeInput {
    pub code: String,
    pub transient_payload: Option<Value>,
}

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct UpdateSettingsInput {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<Value>,
}

impl From<LoginInput> for LoginCredentials {
    fn from(input: LoginInput) -> Self {
        Self {
            identifier: input.identifier,
            password: input.password,
            address: input.address,
            code: input.code,
            resend: input.resend,
        }
    }
}

impl From<RecoveryInput> for RecoveryRequest {
    fn from(input: RecoveryInput) -> Self {
        Self { email: input.email }
    }
}

impl From<RegisterInput> for RegistrationData {
    fn from(input: RegisterInput) -> Self {
        Self {
            email: input.email,
            username: input.username.unwrap_or_default(),
            password: input.password,
            geo_location: input.geo_location,
        }
    }
}

impl From<UpdateSettingsInput> for SettingsData {
    fn from(input: UpdateSettingsInput) -> Self {
        Self {
            method: input.method,
            password: input.password,
            traits: input.traits,
            lookup_secret_confirm: input.lookup_secret_confirm,
            lookup_secret_disable: input.lookup_secret_disable,
            lookup_secret_regenerate: input.lookup_secret_regenerate,
            lookup_secret_reveal: input.lookup_secret_reveal,
            transient_payload: input.transient_payload,
        }
    }
}

impl From<VerifyByLinkInput> for VerifyByLinkRequest {
    fn from(input: VerifyByLinkInput) -> Self {
        Self {
            email: input.email,
            transient_payload: input.transient_payload,
        }
    }
}

impl From<SendVerificationCodeInput> for SendCodeRequest {
    fn from(input: SendVerificationCodeInput) -> Self {
        Self {
            email: input.email,
            transient_payload: input.transient_payload,
        }
    }
}

impl From<SubmitVerificationCodeInput> for SubmitCodeRequest {
    fn from(input: SubmitVerificationCodeInput) -> Self {
        Self {
            code: input.code,
            transient_payload: input.transient_payload,
        }
    }
}
