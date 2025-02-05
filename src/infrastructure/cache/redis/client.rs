use crate::infrastructure::cache::redis::{RedisConfig, RedisServiceError};
use redis::{Client, Connection};
use std::net::TcpStream;
use std::time::Duration;

pub struct RedisClient {
    client: Client,
    config: RedisConfig,
}

impl RedisClient {
    pub fn new(config: RedisConfig) -> Result<Self, RedisServiceError> {
        // 首先测试端口是否可访问
        let host_port = format!(
            "{}:{}",
            config.host.as_ref().unwrap_or(&"127.0.0.1".to_string()),
            config.port.as_ref().unwrap_or(&"6379".to_string())
        );

        TcpStream::connect(&host_port).map_err(|e| {
            RedisServiceError::ConnectionError(format!(
                "无法连接到 Redis 服务器 {}：{}。请确保：\n\
                1. Redis 服务器已启动\n\
                2. 端口号正确\n\
                3. 防火墙未阻止连接",
                host_port, e
            ))
        })?;

        let client = Client::open(config.url.as_str()).map_err(|e| {
            RedisServiceError::ConnectionError(format!("Redis 客户端创建失败：{}", e))
        })?;

        // 测试连接并进行认证
        let mut conn = client.get_connection().map_err(|e| {
            RedisServiceError::ConnectionError(format!("无法获取 Redis 连接：{}", e))
        })?;

        // 如果设置了密码，尝试认证
        if let Some(password) = &config.password {
            redis::cmd("AUTH")
                .arg(password)
                .query::<String>(&mut conn)
                .map_err(|e| {
                    RedisServiceError::ConnectionError(format!(
                        "Redis 认证失败：{}。请检查密码是否正确。",
                        e
                    ))
                })?;
        }

        // 测试连接
        redis::cmd("PING").query::<String>(&mut conn).map_err(|e| {
            RedisServiceError::ConnectionError(format!(
                "Redis PING 测试失败：{}。连接可能不稳定。",
                e
            ))
        })?;

        Ok(Self { client, config })
    }

    pub fn get_connection(&self) -> Result<Connection, RedisServiceError> {
        let mut conn = self
            .client
            .get_connection_with_timeout(Duration::from_secs(self.config.connection_timeout))
            .map_err(|e| {
                RedisServiceError::ConnectionError(format!("获取 Redis 连接失败：{}", e))
            })?;

        // 每次获取新连接时都需要认证
        if let Some(password) = &self.config.password {
            redis::cmd("AUTH")
                .arg(password)
                .query::<String>(&mut conn)
                .map_err(|e| {
                    RedisServiceError::ConnectionError(format!(
                        "Redis 认证失败：{}。请检查密码是否正确。",
                        e
                    ))
                })?;
        }

        Ok(conn)
    }

    // 添加测试连接的方法
    pub fn test_connection(&self) -> Result<(), RedisServiceError> {
        let mut conn = self.get_connection()?;
        redis::cmd("PING").query::<String>(&mut conn).map_err(|e| {
            RedisServiceError::ConnectionError(format!(
                "Redis 连接测试失败：{}。请检查服务器状态。",
                e
            ))
        })?;
        Ok(())
    }
}
