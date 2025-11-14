use anyhow::Context;
use redis::aio::ConnectionManager;
use crate::config::RedisConfig;
use crate::shared::errors::error::Error;

pub async fn create_redis_pool(config: &RedisConfig) -> Result<ConnectionManager, Error> {
    let redis_url = format!(
        "redis://{}:{}@{}:{}/{}",
        config.username,
        config.password,
        config.host,
        config.port,
        config.db
    );

    let redis_client = redis::Client::open(redis_url)?;
    let redis_manager = ConnectionManager::new(redis_client)
        .await?;
    Ok(redis_manager)
}

