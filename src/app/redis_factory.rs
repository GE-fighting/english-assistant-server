use crate::infrastructure::cache::redis::{RedisClient, RedisConfig, RedisOperations};
use std::sync::Arc;

pub struct RedisFactory;

impl RedisFactory {
    pub fn create_redis_client() -> Arc<dyn RedisOperations> {
        let config = RedisConfig::from_env().expect("Failed to load Redis configuration");

        let client = RedisClient::new(config).expect("Failed to create Redis client");

        Arc::new(client)
    }
}
