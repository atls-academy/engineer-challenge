use crate::application::bootstrap::config::KratosConfig;
use crate::domain::ports::{
    identity::IdentityPort, login::AuthenticationPort, recovery::RecoveryPort,
    registration::RegistrationPort, session::SessionPort, settings::SettingsPort,
    verification::VerificationPort,
};
use crate::infrastructure::adapters::kratos::{
    client::KratosClient,
    http::{
        identity::KratosIdentityAdapter, login::KratosAuthenticationAdapter,
        logout::KratosSessionAdapter, recovery::KratosRecoveryAdapter,
        register::KratosRegistrationAdapter, settings::KratosSettingsAdapter,
        verification::KratosVerificationAdapter,
    },
};
use crate::infrastructure::di::adapter_factory::AdapterFactory;
use std::sync::Arc;

pub struct KratosAdapterFactory {
    client: Arc<KratosClient>,
}

impl KratosAdapterFactory {
    pub fn new(config: &KratosConfig) -> Self {
        Self {
            client: Arc::new(KratosClient::new(config)),
        }
    }

    pub fn from_client(client: Arc<KratosClient>) -> Self {
        Self { client }
    }
}

impl AdapterFactory for KratosAdapterFactory {
    fn create_registration_adapter(&self) -> Arc<dyn RegistrationPort> {
        Arc::new(KratosRegistrationAdapter::new(self.client.clone()))
    }

    fn create_authentication_adapter(&self) -> Arc<dyn AuthenticationPort> {
        Arc::new(KratosAuthenticationAdapter::new(self.client.clone()))
    }

    fn create_session_adapter(&self) -> Arc<dyn SessionPort> {
        Arc::new(KratosSessionAdapter::new(self.client.clone()))
    }

    fn create_recovery_adapter(&self) -> Arc<dyn RecoveryPort> {
        Arc::new(KratosRecoveryAdapter::new(self.client.clone()))
    }

    fn create_verification_adapter(&self) -> Arc<dyn VerificationPort> {
        Arc::new(KratosVerificationAdapter::new(self.client.clone()))
    }

    fn create_identity_adapter(&self) -> Arc<dyn IdentityPort> {
        Arc::new(KratosIdentityAdapter::new(self.client.clone()))
    }

    fn create_settings_adapter(&self) -> Arc<dyn SettingsPort> {
        Arc::new(KratosSettingsAdapter::new(self.client.clone()))
    }
}
