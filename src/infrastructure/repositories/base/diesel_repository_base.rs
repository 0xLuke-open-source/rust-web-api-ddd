use crate::infrastructure::database::{DbConnection, DbPool};
use crate::shared::errors::error::Error;

// 可以创建一个通用的仓储基类或工具模块
#[derive(Clone)]
pub struct DieselRepositoryBase {
    pool: DbPool,
}

impl DieselRepositoryBase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn get_connection(&self) -> Result<DbConnection, Error> {
        self.pool.get().map_err(|e| {
            Error::Database(sqlx::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            )))
        })
    }

    pub fn map_diesel_error(&self, e: diesel::result::Error) -> Error {
        Error::Database(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )))
    }
}
