use crate::application::boostrap::config::Config;
use crate::application::usecases::auth::{
    get_current_user::GetCurrentUserUseCase, login::LoginUseCase, logout::LogoutUseCase,
    recovery::RecoveryUseCase, register::RegisterUseCase, settings::UpdateSettingsUseCase,
    verification::VerificationUseCase,
};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::di::{adapter_factory::AdapterFactory, factory::KratosAdapterFactory};
use std::sync::Arc;

pub struct UseCases {
    pub register: RegisterUseCase,
    pub login: LoginUseCase,
    pub logout: LogoutUseCase,
    pub recovery: RecoveryUseCase,
    pub verification: VerificationUseCase,
    pub get_current_user: GetCurrentUserUseCase,
    pub update_settings: UpdateSettingsUseCase,
}

impl UseCases {
    fn new(factory: &dyn AdapterFactory) -> Self {
        Self {
            register: RegisterUseCase::new(factory.create_registration_adapter()),
            login: LoginUseCase::new(factory.create_authentication_adapter()),
            logout: LogoutUseCase::new(factory.create_session_adapter()),
            recovery: RecoveryUseCase::new(factory.create_recovery_adapter()),
            verification: VerificationUseCase::new(factory.create_verification_adapter()),
            get_current_user: GetCurrentUserUseCase::new(factory.create_identity_adapter()),
            update_settings: UpdateSettingsUseCase::new(factory.create_settings_adapter()),
        }
    }
}

#[derive(Clone)]
pub struct AppContainer {
    pub use_cases: Arc<UseCases>,
    pub kratos: Arc<KratosClient>,
}

impl AppContainer {
    pub fn new(config: &Config) -> Result<Self, ContainerError> {
        Self::validate_config(config)?;

        let kratos = Arc::new(KratosClient::new(&config.kratos));
        let factory = KratosAdapterFactory::from_client(kratos.clone());
        let use_cases = Arc::new(UseCases::new(&factory));

        Ok(Self { use_cases, kratos })
    }

    fn validate_config(config: &Config) -> Result<(), ContainerError> {
        if config.kratos.public_url.is_empty() {
            return Err(ContainerError::InvalidConfig(
                "Kratos public URL cannot be empty".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Initialization failed: {0}")]
    Initialization(String),
    #[error("Factory creation failed: {0}")]
    FactoryCreation(String),
}
