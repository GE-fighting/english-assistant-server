use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            // 从默认文件开始
            .add_source(File::with_name("config/default"))
            // 如果有本地配置文件，则合并它
            .add_source(File::with_name("config/local").required(false))
            // 添加环境变量支持，例如 `APP_SERVER__PORT=5001` 会设置 `Settings.server.port`
            .add_source(Environment::with_prefix("app").separator("__"))
            .build()?;

        // 你可以反序列化整个配置
        s.try_deserialize()
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
}
