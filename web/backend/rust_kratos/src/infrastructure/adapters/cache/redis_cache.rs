use crate::domain::errors::DomainError;
use redis::AsyncCommands;
use std::time::Duration;

#[derive(Clone)]
pub struct RedisCache {
    connection: redis::aio::ConnectionManager,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> Result<Self, DomainError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        let connection = redis::aio::ConnectionManager::new(client)
            .await
            .map_err(|e| DomainError::ServiceUnavailable(e.to_string()))?;

        Ok(Self { connection })
    }

    pub async fn new_with_retry(
        redis_url: &str,
        max_retries: u32,
        retry_delay: Duration,
    ) -> Result<Self, DomainError> {
        let mut last_err: Option<DomainError> = None;

        for attempt in 1..=max_retries {
            match Self::new(redis_url).await {
                Ok(cache) => {
                    tracing::info!(attempt, "Redis connected");
                    return Ok(cache);
                }
                Err(e) => {
                    tracing::warn!(
                        attempt,
                        max = max_retries,
                        error = %e,
                        "Redis unavailable, retrying in {:?}", retry_delay
                    );
                    last_err = Some(e);

                    if attempt < max_retries {
                        tokio::time::sleep(retry_delay).await;
                    }
                }
            }
        }

        Err(last_err.expect("no error captured but Redis never connected"))
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.connection.clone();
        conn.get::<_, String>(key).await.ok()
    }

    pub fn connection(&self) -> redis::aio::ConnectionManager {
        self.connection.clone()
    }

    pub async fn set_ex(&self, key: &str, value: &str, ttl_seconds: u64) {
        let mut conn = self.connection.clone();
        let _: Result<(), _> = conn.set_ex(key, value, ttl_seconds).await;
    }

    pub async fn delete(&self, key: &str) {
        let mut conn = self.connection.clone();
        let _: Result<(), _> = conn.del(key).await;
    }
}
