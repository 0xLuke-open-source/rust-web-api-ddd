use std::net::SocketAddr;

use anyhow::Context;
use tokio::net::TcpListener;

use crate::{
    api::create_router,
    config::Config,
    infrastructure::{
        create_db_pool,
        database::create_diesel_pool,
        database::redis::create_redis_pool,
    },
};
use crate::api::AppState;
use crate::api::repository::repository_registry::RepositoryRegistry;

/// 应用程序启动与管理结构体
pub struct Application;

impl Application {
    /// 构建应用实例并启动服务
    /// 包含数据库连接池初始化、仓储创建、路由设置及服务器启动
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        // 1. 初始化数据库连接池
        // SQLx 连接池
        let _db_pool = create_db_pool(&config.database)  // SQLx 池（如需使用保留）
            .await
            .context("Failed to create SQLx database pool")?;

        // Diesel 连接池
        let diesel_pool = create_diesel_pool(&config.database)
            .context("Failed to create Diesel database pool")?;

        // 初始化Redis连接池
        let redis_manager = create_redis_pool(&config.redis)
            .await
            .context("Failed to create Redis connection pool")?;

        // 2. 创建仓储注册表（通用化核心：新增仓储只需修改 RepositoryRegistry）
        let repo_registry = RepositoryRegistry::new(diesel_pool);

        // 3. 创建应用状态（自动包含所有服务）
        let app_state = AppState::new(repo_registry, redis_manager,config.clone());

        // 4. 创建通用路由（无需修改，自动包含所有模块路由）
        let router = create_router(app_state);

        // 5. 启动服务器
        let address = SocketAddr::from(([0, 0, 0, 0], config.server.port));
        let listener = TcpListener::bind(address)
            .await
            .context(format!("Failed to bind to address: {}", address))?;

        tracing::info!("Server starting on {}", address);
        axum::serve(listener, router)
            .await
            .context("Server failed to start")?;

        Ok(Self)
    }

    /// 服务运行直到停止（当前因服务在 build 中启动，此处仅作为占位返回）
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        Ok(())
    }
}