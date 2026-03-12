use crate::infrastructure::adapters::cache::redis_cache::RedisCache;
use crate::infrastructure::adapters::graphql::rate_limit::config::RateLimitRule;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RateLimiter {
    cache: RedisCache,
}

impl RateLimiter {
    pub fn new(cache: RedisCache) -> Self {
        Self { cache }
    }

    pub async fn check(
        &self,
        operation: &str,
        identifier: &str,
        rule: &RateLimitRule,
    ) -> Result<(), String> {
        let key = format!("rate_limit:{}:{}", operation, identifier);
        let mut conn = self.cache.connection();

        let count: u64 = conn.incr(&key, 1).await.unwrap_or(0);

        if count == 1 {
            let _: Result<(), _> = conn.expire(&key, rule.window_seconds as i64).await;
        }

        if count > rule.max_requests {
            return Err(format!(
                "Rate limit exceeded for '{}'. Max {} requests per {} seconds.",
                operation, rule.max_requests, rule.window_seconds
            ));
        }

        Ok(())
    }
}
