use crate::api::repository::repository_registry::RepositoryRegistry;
use crate::application::services::menu_service::MenuService;
use crate::application::services::user_service::UserService;
use crate::infrastructure::repositories::{DieselMenuRepository, DieselUserRepository};
use std::sync::Arc;
use crate::application::services::role_service::RoleService;
use crate::infrastructure::repositories::diesel_role_repository::DieselRoleRepository;

/// 应用全局状态，持有服务层实例
/// 新增服务时只需在此添加字段
#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService<DieselUserRepository>>,
    pub menu_service: Arc<MenuService<DieselMenuRepository>>,
    pub role_service: Arc<RoleService<DieselRoleRepository>>,
    // 未来新增服务在此添加，例：
    // pub order_service: Arc<OrderService>,
}

impl AppState {
    /// 从仓储注册表创建所有服务实例
    pub fn new(repos: RepositoryRegistry) -> Self {
        Self {
            user_service: Arc::new(UserService::new(repos.user)),
            menu_service: Arc::new(MenuService::new(repos.menu)),
            role_service: Arc::new(RoleService::new(repos.role)),
            // 新增服务初始化，例：
            // order_service: Arc::new(OrderService::new(repos.order)),
        }
    }
}
