use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
