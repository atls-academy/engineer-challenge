use crate::application::bootstrap::config::RateLimitsConfig;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RateLimitRule {
    pub max_requests: u64,
    pub window_seconds: u64,
}

impl RateLimitRule {
    pub fn new(max_requests: u64, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

pub fn rules_from_config(cfg: &RateLimitsConfig) -> HashMap<&'static str, RateLimitRule> {
    let mut rules = HashMap::new();

    rules.insert(
        "login",
        RateLimitRule::new(cfg.login.max_requests, cfg.login.window_seconds),
    );
    rules.insert(
        "register",
        RateLimitRule::new(cfg.register.max_requests, cfg.register.window_seconds),
    );
    rules.insert(
        "recovery",
        RateLimitRule::new(cfg.recovery.max_requests, cfg.recovery.window_seconds),
    );
    rules.insert(
        "settings",
        RateLimitRule::new(cfg.settings.max_requests, cfg.settings.window_seconds),
    );
    rules.insert(
        "verification",
        RateLimitRule::new(
            cfg.verification.max_requests,
            cfg.verification.window_seconds,
        ),
    );
    rules.insert(
        "currentUser",
        RateLimitRule::new(
            cfg.current_user.max_requests,
            cfg.current_user.window_seconds,
        ),
    );
    rules.insert(
        "logout",
        RateLimitRule::new(cfg.logout.max_requests, cfg.logout.window_seconds),
    );
    rules
}
