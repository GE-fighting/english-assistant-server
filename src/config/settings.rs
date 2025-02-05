use anyhow::Result;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use std::env;

use super::app_config::{AppConfig, ServerConfig};
use super::database::DatabaseConfig;
use super::llm_config::LLMConfig;

static SETTINGS: OnceCell<Settings> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Settings {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

impl Settings {
    pub fn global() -> &'static Settings {
        SETTINGS.get_or_init(|| Self::new().expect("Failed to initialize settings"))
    }

    fn new() -> Result<Self, String> {
        // Initialize environment variables
        // Try loading from .env file first, then .env.local if that fails
        if dotenv::from_filename(".env").is_err() {
            if dotenv::from_filename(".env.local").is_err() {
                dotenv().ok(); // Default to loading .env file
            }
        }

        // Server configuration
        let server_config = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| "Invalid SERVER_PORT")?,
        };

        let app_config = AppConfig {
            server: server_config,
        };

        // Database configuration
        let database_config = DatabaseConfig {
            host: env::var("DB_HOST").map_err(|_| "DB_HOST not set")?,
            port: env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .map_err(|_| "Invalid DB_PORT")?,
            username: env::var("DB_USER").map_err(|_| "DB_USER not set")?,
            password: env::var("DB_PASSWORD").map_err(|_| "DB_PASSWORD not set")?,
            database_name: env::var("DB_NAME").unwrap_or_else(|_| "english_assistant".to_string()),
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .map_err(|_| "Invalid DB_MAX_CONNECTIONS")?,
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .map_err(|_| "Invalid DB_MIN_CONNECTIONS")?,
        };

        // LLM configuration

        Ok(Settings {
            app: app_config,
            database: database_config,
        })
    }
}
