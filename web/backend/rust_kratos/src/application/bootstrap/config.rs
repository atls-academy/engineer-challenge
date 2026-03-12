use serde::Deserialize;
use std::env;
use tracing::info;

#[derive(Debug, Clone, Copy)]
pub enum Environment {
    Development,
    Production,
    DockerLocal,
}

impl Environment {
    pub fn from_env() -> Self {
        match env::var("APP_ENV")
            .unwrap_or_else(|_| String::from("development"))
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            "docker_local" => Environment::DockerLocal,
            _ => Environment::Development,
        }
    }

    pub fn config_filename(&self) -> &str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::DockerLocal => "docker_local",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub kratos: KratosConfig,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub rate_limits: RateLimitsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_secs: u64,
    #[serde(default = "default_redis_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_redis_retry_delay")]
    pub retry_delay_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KratosConfig {
    pub admin_url: String,
    pub public_url: String,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,
    #[serde(default = "default_pool_idle_timeout")]
    pub pool_idle_timeout_secs: u64,
    #[serde(default = "default_pool_max_idle")]
    pub pool_max_idle_per_host: usize,
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_retry_delay")]
    pub retry_delay_ms: u64,
    #[serde(default = "default_accept_invalid_certs")]
    pub accept_invalid_certs: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitRuleConfig {
    pub max_requests: u64,
    pub window_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitsConfig {
    pub login: RateLimitRuleConfig,
    pub register: RateLimitRuleConfig,
    pub recovery: RateLimitRuleConfig,
    pub settings: RateLimitRuleConfig,
    pub verification: RateLimitRuleConfig,
    pub current_user: RateLimitRuleConfig,
    pub logout: RateLimitRuleConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_cors_max_age")]
    pub cors_max_age: usize,
    #[serde(default = "default_cors_allowed_origins")]
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let environment = Environment::from_env();
        let config_filename = environment.config_filename();
        let config_path = format!("config/app/{}", config_filename);

        info!(path = %format!("{}.toml", config_path), "Loading config file");

        let builder = config::Config::builder()
            .add_source(
                config::File::with_name(&config_path)
                    .required(true)
                    .format(config::FileFormat::Toml),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            );

        builder.build()?.try_deserialize()
    }
}

fn default_cache_ttl() -> u64 {
    300
}
fn default_redis_max_retries() -> u32 {
    5
}
fn default_redis_retry_delay() -> u64 {
    2000
}
fn default_timeout() -> u64 {
    120
}
fn default_connect_timeout() -> u64 {
    30
}
fn default_pool_idle_timeout() -> u64 {
    120
}
fn default_pool_max_idle() -> usize {
    10
}
fn default_max_retries() -> u32 {
    3
}
fn default_retry_delay() -> u64 {
    1000
}
fn default_accept_invalid_certs() -> bool {
    false
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_cors_max_age() -> usize {
    3600
}
fn default_cors_allowed_origins() -> Vec<String> {
    vec!["http://localhost:3000".to_string()]
}
