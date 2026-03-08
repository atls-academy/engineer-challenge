use crate::application::boostrap::config::KratosConfig;
use crate::domain::ports::{
    identity::IdentityPort, login::AuthenticationPort, recovery::RecoveryPort,
    registration::RegistrationPort, session::SessionPort, settings::SettingsPort,
    verification::VerificationPort,
};
use crate::infrastructure::adapters::kratos::client::KratosClient;
use crate::infrastructure::adapters::kratos::http::identity::KratosIdentityAdapter;
use crate::infrastructure::adapters::kratos::http::login::KratosAuthenticationAdapter;
use crate::infrastructure::adapters::kratos::http::logout::KratosSessionAdapter;
use crate::infrastructure::adapters::kratos::http::recovery::KratosRecoveryAdapter;
use crate::infrastructure::adapters::kratos::http::register::KratosRegistrationAdapter;
use crate::infrastructure::adapters::kratos::http::settings::KratosSettingsAdapter;
use crate::infrastructure::adapters::kratos::http::verification::KratosVerificationAdapter;
use crate::infrastructure::di::adapter_factory::AdapterFactory;
use std::sync::Arc;

pub struct KratosAdapterFactory {
    kratos_client: Arc<KratosClient>,
}

impl KratosAdapterFactory {
    pub fn new(config: &KratosConfig) -> Result<Self, FactoryError> {
        Ok(Self {
            kratos_client: Arc::new(KratosClient::new(config)),
        })
    }

    pub fn from_client(client: Arc<KratosClient>) -> Self {
        Self {
            kratos_client: client,
        }
    }
}

impl AdapterFactory for KratosAdapterFactory {
    fn create_registration_adapter(&self) -> Arc<dyn RegistrationPort> {
        Arc::new(KratosRegistrationAdapter::new(self.kratos_client.clone()))
    }
    fn create_authentication_adapter(&self) -> Arc<dyn AuthenticationPort> {
        Arc::new(KratosAuthenticationAdapter::new(self.kratos_client.clone()))
    }
    fn create_session_adapter(&self) -> Arc<dyn SessionPort> {
        Arc::new(KratosSessionAdapter::new(self.kratos_client.clone()))
    }
    fn create_recovery_adapter(&self) -> Arc<dyn RecoveryPort> {
        Arc::new(KratosRecoveryAdapter::new(self.kratos_client.clone()))
    }
    fn create_verification_adapter(&self) -> Arc<dyn VerificationPort> {
        Arc::new(KratosVerificationAdapter::new(self.kratos_client.clone()))
    }
    fn create_identity_adapter(&self) -> Arc<dyn IdentityPort> {
        Arc::new(KratosIdentityAdapter::new(self.kratos_client.clone()))
    }
    fn create_settings_adapter(&self) -> Arc<dyn SettingsPort> {
        Arc::new(KratosSettingsAdapter::new(self.kratos_client.clone()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FactoryError {
    #[error("Failed to create Kratos client: {0}")]
    KratosClientCreation(String),
}
