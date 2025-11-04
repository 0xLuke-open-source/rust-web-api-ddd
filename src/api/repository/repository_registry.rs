use crate::infrastructure::repositories::{DieselMenuRepository, DieselUserRepository};
use crate::infrastructure::repositories::diesel_role_repository::DieselRoleRepository;

/// 仓储注册表，集中管理所有数据仓储实例
/// 新增仓储时只需在此添加字段并扩展构造方法
#[derive(Clone)]
pub struct RepositoryRegistry {
    pub user: DieselUserRepository,
    pub menu: DieselMenuRepository,
    pub role: DieselRoleRepository,
    // 未来新增仓储在此添加，例：
    // pub order: DieselOrderRepository,
}

impl RepositoryRegistry {
    /// 从数据库连接池创建所有仓储实例
    pub fn new(diesel_pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>) -> Self {
        Self {
            user: DieselUserRepository::new(diesel_pool.clone()),
            menu: DieselMenuRepository::new(diesel_pool.clone()),
            role: DieselRoleRepository::new(diesel_pool.clone()),
            // 新增仓储初始化，例：
            // order: DieselOrderRepository::new(diesel_pool),
        }
    }
}