use dotenv::dotenv;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::env;

static SETTINGS: OnceCell<Settings> = OnceCell::new();

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    lingyiwanwu_api_key: String,
}

impl Settings {
    pub fn global() -> &'static Settings {
        SETTINGS.get_or_init(|| Self::new().expect("Failed to initialize settings"))
    }

    fn new() -> Result<Self, String> {
        // 初始化环境变量
        // 尝试从 .env 文件加载环境变量，如果失败则尝试从 .env.local 文件加载
        if dotenv::from_filename(".env").is_err() {
            if dotenv::from_filename(".env.local").is_err() {
                dotenv().ok(); // 默认加载 .env 文件
            }
        }

        // 从环境变量读取配置
        let server_config = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| "Invalid SERVER_PORT")?,
        };

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

        let lingyiwanwu_api_key =
            env::var("LLM_API_KEY").map_err(|_| "LINGYIWANWU_API_KEY must be set")?;

        Ok(Settings {
            server: server_config,
            database: database_config,
            lingyiwanwu_api_key,
        })
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database_name
        )
    }

    pub fn get_lingyiwanwu_api_key(&self) -> &str {
        &self.lingyiwanwu_api_key
    }
}
