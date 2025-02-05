mod client;
mod config;
mod error;
mod operations;

pub use client::RedisClient;
pub use config::RedisConfig;
pub use error::RedisServiceError;
pub use operations::RedisOperations;

use once_cell::sync::OnceCell;

static REDIS_INSTANCE: OnceCell<RedisClient> = OnceCell::new();

/// Initialize global Redis instance
pub async fn init() -> Result<(), RedisServiceError> {
    let config = RedisConfig::from_env()?;
    let client = RedisClient::new(config)?;
    REDIS_INSTANCE.set(client).map_err(|_| {
        RedisServiceError::InitializationError("Redis instance already initialized".into())
    })?;
    Ok(())
}

/// Get global Redis instance
pub fn get_instance() -> Result<&'static RedisClient, RedisServiceError> {
    REDIS_INSTANCE.get().ok_or_else(|| {
        RedisServiceError::UninitializedError("Redis instance not initialized".into())
    })
}
