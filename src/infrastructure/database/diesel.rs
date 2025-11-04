use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::config::DatabaseConfig;
use crate::shared::errors::error::Error;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn create_diesel_pool(config: &DatabaseConfig) -> Result<DbPool, Error> {
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database_name
    );

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(config.max_connections)
        .build(manager)
        .map_err(|e| Error::Database(sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))))?;

    Ok(pool)
}
