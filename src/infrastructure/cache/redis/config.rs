use crate::infrastructure::cache::redis::RedisServiceError;
use log::{debug, warn};
use redis::ToRedisArgs;
use std::env;

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub host: Option<String>,
    pub port: Option<String>,
    pub password: Option<String>,
    pub pool_size: u32,
    pub connection_timeout: u64,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, RedisServiceError> {
        // 读取并记录环境变量
        let password = env::var("REDIS_PASSWORD").ok();
        let host = env::var("REDIS_HOST").ok();
        let port = env::var("REDIS_PORT").ok();

        debug!("Redis配置加载状态:");
        debug!("REDIS_HOST: {:?}", host);
        debug!("REDIS_PORT: {:?}", port);
        debug!(
            "REDIS_PASSWORD: {}",
            if password.is_some() {
                "已设置"
            } else {
                "未设置"
            }
        );

        if host.is_none() {
            warn!("REDIS_HOST 未设置，使用默认值: 127.0.0.1");
        }
        if port.is_none() {
            warn!("REDIS_PORT 未设置，使用默认值: 6379");
        }

        // 构建Redis URL
        let url = if let Some(pass) = &password {
            format!(
                "redis://:{}@{}:{}",
                pass,
                host.as_ref().unwrap_or(&"127.0.0.1".to_string()),
                port.as_ref().unwrap_or(&"6379".to_string())
            )
        } else {
            format!(
                "redis://{}:{}",
                host.as_ref().unwrap_or(&"127.0.0.1".to_string()),
                port.as_ref().unwrap_or(&"6379".to_string())
            )
        };

        debug!("构建的 Redis URL: {}", url);

        let pool_size = env::var("REDIS_POOL_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);
        let connection_timeout = env::var("REDIS_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        debug!("Redis连接池大小: {}", pool_size);
        debug!("Redis连接超时: {}秒", connection_timeout);

        Ok(Self {
            url,
            host,
            port,
            password,
            pool_size,
            connection_timeout,
        })
    }

    // 添加测试配置方法，用于单元测试
    #[cfg(test)]
    pub fn test_config() -> Self {
        Self {
            url: "redis://115.120.238.73".to_string(),
            host: Some("115.120.238.73".to_string()),
            port: Some("6379".to_string()),
            password: Some("123456".to_string()),
            pool_size: 1,
            connection_timeout: 1,
        }
    }

    // 添加配置验证方法
    pub fn validate(&self) -> Result<(), RedisServiceError> {
        debug!("验证 Redis 配置...");
        if self.pool_size == 0 {
            return Err(RedisServiceError::ConfigError("连接池大小不能为0".into()));
        }
        if self.connection_timeout == 0 {
            return Err(RedisServiceError::ConfigError("连接超时不能为0".into()));
        }
        debug!("Redis 配置验证通过");
        Ok(())
    }
}
