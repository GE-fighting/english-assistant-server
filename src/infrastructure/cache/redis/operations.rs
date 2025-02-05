use crate::infrastructure::cache::redis::{RedisClient, RedisServiceError};
use async_trait::async_trait;

#[async_trait]
pub trait RedisOperations: Sync + Send {
    async fn set_key(
        &self,
        key: &str,
        value: &str,
        expiration_secs: Option<usize>,
    ) -> Result<(), RedisServiceError>;
    async fn get_key(&self, key: &str) -> Result<Option<String>, RedisServiceError>;
    async fn delete_key(&self, key: &str) -> Result<bool, RedisServiceError>;
    async fn exists_key(&self, key: &str) -> Result<bool, RedisServiceError>;
    async fn set_key_nx(&self, key: &str, value: &str) -> Result<bool, RedisServiceError>;
    async fn increment(&self, key: &str) -> Result<i64, RedisServiceError>;
    async fn decrement(&self, key: &str) -> Result<i64, RedisServiceError>;
}

#[async_trait]
impl RedisOperations for RedisClient {
    async fn set_key(
        &self,
        key: &str,
        value: &str,
        expiration_secs: Option<usize>,
    ) -> Result<(), RedisServiceError> {
        let mut conn = self.get_connection()?;
        match expiration_secs {
            Some(secs) => redis::cmd("SETEX")
                .arg(key)
                .arg(secs)
                .arg(value)
                .query(&mut conn)
                .map_err(|e| RedisServiceError::OperationError(e.to_string())),
            None => redis::cmd("SET")
                .arg(key)
                .arg(value)
                .query(&mut conn)
                .map_err(|e| RedisServiceError::OperationError(e.to_string())),
        }
    }

    async fn get_key(&self, key: &str) -> Result<Option<String>, RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("GET")
            .arg(key)
            .query(&mut conn)
            .map_err(|e| RedisServiceError::OperationError(e.to_string()))
    }

    async fn delete_key(&self, key: &str) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("DEL")
            .arg(key)
            .query(&mut conn)
            .map_err(|e| RedisServiceError::OperationError(e.to_string()))
    }

    async fn exists_key(&self, key: &str) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("EXISTS")
            .arg(key)
            .query(&mut conn)
            .map_err(|e| RedisServiceError::OperationError(e.to_string()))
    }

    async fn set_key_nx(&self, key: &str, value: &str) -> Result<bool, RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("SETNX")
            .arg(key)
            .arg(value)
            .query(&mut conn)
            .map_err(|e| RedisServiceError::OperationError(e.to_string()))
    }

    async fn increment(&self, key: &str) -> Result<i64, RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("INCR")
            .arg(key)
            .query(&mut conn)
            .map_err(|e| RedisServiceError::OperationError(e.to_string()))
    }

    async fn decrement(&self, key: &str) -> Result<i64, RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("DECR")
            .arg(key)
            .query(&mut conn)
            .map_err(|e| RedisServiceError::OperationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::cache::redis::RedisConfig;

    #[tokio::test]
    async fn test_redis_operations() -> Result<(), RedisServiceError> {
        let config = RedisConfig::from_env()?;
        let client = RedisClient::new(config)?;

        // Test set and get
        client.set_key("test_key", "test_value", None).await?;
        let value = client.get_key("test_key").await?;
        assert_eq!(value, Some("test_value".to_string()));

        // Test exists
        let exists = client.exists_key("test_key").await?;
        assert!(exists);

        // Test delete
        client.delete_key("test_key").await?;
        let exists = client.exists_key("test_key").await?;
        assert!(!exists);

        Ok(())
    }
}
