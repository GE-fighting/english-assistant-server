use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RedisServiceError {
    ConnectionError(String),
    OperationError(String),
    ConfigError(String),
    InitializationError(String),
    UninitializedError(String),
}

impl fmt::Display for RedisServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RedisServiceError::ConnectionError(msg) => write!(f, "Redis connection error: {}", msg),
            RedisServiceError::OperationError(msg) => write!(f, "Redis operation error: {}", msg),
            RedisServiceError::ConfigError(msg) => write!(f, "Redis configuration error: {}", msg),
            RedisServiceError::InitializationError(msg) => {
                write!(f, "Redis initialization error: {}", msg)
            }
            RedisServiceError::UninitializedError(msg) => {
                write!(f, "Redis uninitialized error: {}", msg)
            }
        }
    }
}

impl Error for RedisServiceError {}
