use crate::application::bootstrap::config::{Config, RateLimitsConfig};
use crate::application::commands::{
    account::{
        recovery::RecoveryCommandHandler, settings::UpdateSettingsCommandHandler,
        verification::VerificationCommandHandler,
    },
    auth::{login::LoginCommandHandler, logout::LogoutCommandHandler},
    identity::register::RegisterCommandHandler,
};
use crate::application::queries::get_current_user::GetCurrentUserQueryHandler;
use crate::infrastructure::di::adapter_factory::AdapterFactory;
use crate::infrastructure::{
    adapters::cache::redis_cache::RedisCache, adapters::kratos::client::KratosClient,
    di::factory::KratosAdapterFactory,
};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

pub struct Commands {
    pub login: LoginCommandHandler,
    pub logout: LogoutCommandHandler,
    pub register: RegisterCommandHandler,
    pub recovery: RecoveryCommandHandler,
    pub update_settings: UpdateSettingsCommandHandler,
    pub verification: VerificationCommandHandler,
}

pub struct Queries {
    pub get_current_user: GetCurrentUserQueryHandler,
}

pub struct UseCases {
    pub commands: Commands,
    pub queries: Queries,
}

impl UseCases {
    pub fn new(factory: &dyn AdapterFactory) -> Self {
        Self {
            commands: Commands {
                login: LoginCommandHandler::new(factory.create_authentication_adapter()),
                logout: LogoutCommandHandler::new(factory.create_session_adapter()),
                register: RegisterCommandHandler::new(factory.create_registration_adapter()),
                recovery: RecoveryCommandHandler::new(factory.create_recovery_adapter()),
                update_settings: UpdateSettingsCommandHandler::new(
                    factory.create_settings_adapter(),
                ),
                verification: VerificationCommandHandler::new(
                    factory.create_verification_adapter(),
                ),
            },
            queries: Queries {
                get_current_user: GetCurrentUserQueryHandler::new(
                    factory.create_identity_adapter(),
                ),
            },
        }
    }
}

#[derive(Clone)]
pub struct AppContainer {
    pub use_cases: Arc<UseCases>,
    pub cache: RedisCache,
    pub rate_limits: RateLimitsConfig,
    kratos: Arc<KratosClient>,
}

impl AppContainer {
    pub async fn new(config: &Config) -> Result<Self, ContainerError> {
        Self::validate_config(config)?;

        let kratos = Arc::new(KratosClient::new(&config.kratos));
        kratos
            .wait_until_ready()
            .await
            .map_err(|e| ContainerError::Initialization(format!("Kratos unavailable: {e}")))?;

        let cache = RedisCache::new_with_retry(
            &config.redis.url,
            config.redis.max_retries,
            Duration::from_millis(config.redis.retry_delay_ms),
        )
        .await
        .map_err(|e| ContainerError::Initialization(format!("Redis unavailable: {e}")))?;

        let factory = KratosAdapterFactory::from_client(
            kratos.clone(),
            cache.clone(),
            config.redis.cache_ttl_secs,
        );

        Ok(Self {
            use_cases: Arc::new(UseCases::new(&factory)),
            cache,
            rate_limits: config.rate_limits.clone(),
            kratos,
        })
    }

    pub fn use_cases(&self) -> Arc<UseCases> {
        self.use_cases.clone()
    }

    pub fn kratos_client(&self) -> Arc<KratosClient> {
        self.kratos.clone()
    }

    fn validate_config(config: &Config) -> Result<(), ContainerError> {
        if config.kratos.public_url.is_empty() {
            return Err(ContainerError::InvalidConfig(
                "Kratos public URL cannot be empty".into(),
            ));
        }
        if config.redis.url.is_empty() {
            return Err(ContainerError::InvalidConfig(
                "Redis URL cannot be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Initialization failed: {0}")]
    Initialization(String),
    #[error("Factory creation failed: {0}")]
    FactoryCreation(String),
}
