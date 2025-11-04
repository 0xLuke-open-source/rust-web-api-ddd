use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use crate::config::DatabaseConfig;

pub type DbPool = MySqlPool;

pub async fn create_db_pool(config: &DatabaseConfig) -> Result<DbPool, sqlx::Error> {
    let connection_string = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database_name
    );

    MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&connection_string)
        .await
}