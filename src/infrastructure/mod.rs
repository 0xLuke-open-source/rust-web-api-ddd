// Infrastructure 层 (infrastructure/)
// 提供技术实现细节和外部资源访问：
// persistence/: 数据库访问实现，包括ORM配置和查询逻辑
// external/: 第三方服务集成，如邮件服务、支付网关等
// config/: 应用配置管理，环境变量处理
mod config;
pub mod database;
mod external;
mod persistence;
pub mod repositories;

pub use database::sqlx::{DbPool, create_db_pool};
