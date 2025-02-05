use crate::config::DatabaseConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database_name
    );

    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect(&database_url)
        .await
}
