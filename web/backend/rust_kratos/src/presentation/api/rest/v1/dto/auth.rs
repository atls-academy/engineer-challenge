use crate::domain::entities::user_profile::UserProfile;
use crate::domain::errors::DomainError;
use crate::domain::ports::inbound::login::LoginCredentials;
use crate::domain::ports::inbound::recovery::RecoveryRequest;
use crate::domain::ports::inbound::registration::RegistrationData;
use crate::domain::ports::inbound::settings::SettingsData;
use crate::domain::ports::inbound::verification::{
    SendCodeRequest, SubmitCodeRequest, VerifyByLinkRequest,
};
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::password::Password;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub username: Option<String>,
    pub password: String,
    pub geo_location: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub identifier: String,
    pub password: String,
    pub address: Option<String>,
    pub code: Option<String>,
    pub resend: Option<String>,
}

#[derive(Deserialize)]
pub struct RecoveryDto {
    pub email: String,
}

#[derive(Deserialize)]
pub struct VerifyByLinkDto {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(Deserialize)]
pub struct SendVerificationCodeDto {
    pub email: String,
    pub transient_payload: Option<Value>,
}

#[derive(Deserialize)]
pub struct SubmitVerificationCodeDto {
    pub code: String,
    pub transient_payload: Option<Value>,
}

#[derive(Deserialize)]
pub struct UpdateSettingsDto {
    pub method: String,
    pub password: Option<String>,
    pub traits: Option<Value>,
    pub lookup_secret_confirm: Option<bool>,
    pub lookup_secret_disable: Option<bool>,
    pub lookup_secret_regenerate: Option<bool>,
    pub lookup_secret_reveal: Option<bool>,
    pub transient_payload: Option<Value>,
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub geo_location: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub state: Option<String>,
    pub active: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

impl From<UserProfile> for UserProfileResponse {
    fn from(p: UserProfile) -> Self {
        Self {
            id: p.id,
            email: p.email,
            username: p.username,
            geo_location: p.geo_location,
            created_at: p.created_at,
            updated_at: p.updated_at,
            state: p.state,
            active: p.active,
            expires_at: p.expires_at,
        }
    }
}

impl TryFrom<LoginDto> for LoginCredentials {
    type Error = DomainError;

    fn try_from(dto: LoginDto) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: Email::new(dto.identifier)?,
            password: Password::new(dto.password)?,
            address: dto.address,
            code: dto.code,
            resend: dto.resend,
        })
    }
}

impl TryFrom<RecoveryDto> for RecoveryRequest {
    type Error = DomainError;

    fn try_from(dto: RecoveryDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(dto.email)?,
        })
    }
}

impl TryFrom<RegisterDto> for RegistrationData {
    type Error = DomainError;

    fn try_from(dto: RegisterDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(dto.email)?,
            username: dto.username.unwrap_or_default(),
            password: Password::new(dto.password)?,
            geo_location: dto.geo_location,
        })
    }
}

impl TryFrom<UpdateSettingsDto> for SettingsData {
    type Error = DomainError;

    fn try_from(dto: UpdateSettingsDto) -> Result<Self, Self::Error> {
        Ok(Self {
            method: dto.method,
            password: dto.password.map(Password::new).transpose()?,
            traits: dto.traits,
            lookup_secret_confirm: dto.lookup_secret_confirm,
            lookup_secret_disable: dto.lookup_secret_disable,
            lookup_secret_regenerate: dto.lookup_secret_regenerate,
            lookup_secret_reveal: dto.lookup_secret_reveal,
            transient_payload: dto.transient_payload,
        })
    }
}

impl TryFrom<VerifyByLinkDto> for VerifyByLinkRequest {
    type Error = DomainError;

    fn try_from(dto: VerifyByLinkDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(dto.email)?,
            transient_payload: dto.transient_payload,
        })
    }
}

impl TryFrom<SendVerificationCodeDto> for SendCodeRequest {
    type Error = DomainError;

    fn try_from(dto: SendVerificationCodeDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(dto.email)?,
            transient_payload: dto.transient_payload,
        })
    }
}

impl From<SubmitVerificationCodeDto> for SubmitCodeRequest {
    fn from(dto: SubmitVerificationCodeDto) -> Self {
        Self {
            code: dto.code,
            transient_payload: dto.transient_payload,
        }
    }
}
