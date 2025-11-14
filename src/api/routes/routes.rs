use crate::api::handlers::{
    create_user_handler, delete_user_handler, get_all_user_handler, get_user_handler,
    update_user_handler,
};
use crate::api::menu_handlers::{create_menu_handler, get_all_menus, get_menu_by_id};
use crate::api::role_handlers::create_role_handler;
use crate::api::{AppState, health_handlers::health_check_handler, login_handler, logout_handler};
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;
use axum::middleware::from_fn_with_state;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use crate::api::middleware::AuthMiddleware;

/// 路由注册器 trait，每个模块实现此 trait 注册自己的路由
pub trait RouteRegistrar {
    /// 注册模块路由，返回带有路径前缀的路由集
    fn register_routes() -> (String, Router<Arc<AppState>>);
}

/// 路由注册表，收集所有模块的路由
struct RouteRegistry;

impl RouteRegistry {
    /// 收集所有模块路由并组合
    fn collect_routes() -> Router<Arc<AppState>> {
        let mut router = Router::new().route("/health", get(health_check_handler));

        // 注册用户模块路由
        let (user_prefix, user_routes) = UserRouteRegistrar::register_routes();
        router = router.nest(&user_prefix, user_routes);

        // 注册菜单模块路由
        let (menu_prefix, menu_routes) = MenuRouteRegistrar::register_routes();
        router = router.nest(&menu_prefix, menu_routes);

        // 注册角色模块路由
        let (role_prefix, role_routes) = RoleRouteRegistrar::register_routes();
        router = router.nest(&role_prefix, role_routes);

        // 新增模块时在此添加注册，例：
        // let (order_prefix, order_routes) = OrderRouteRegistrar::register_routes();
        // router = router.nest(&order_prefix, order_routes);

        router
    }
}

/// 用户模块路由注册器
struct UserRouteRegistrar;
impl RouteRegistrar for UserRouteRegistrar {
    fn register_routes() -> (String, Router<Arc<AppState>>) {
        (
            "/users".to_string(),
            Router::new()
                .route("/add", post(create_user_handler))
                .route("/{id}", get(get_user_handler))
                .route("/{id}", put(update_user_handler))
                .route("/{id}", delete(delete_user_handler))
                .route("/list", get(get_all_user_handler))
                .route("/login", post(login_handler)),
        )
    }
}

/// 菜单模块路由注册器
struct MenuRouteRegistrar;
impl RouteRegistrar for MenuRouteRegistrar {
    fn register_routes() -> (String, Router<Arc<AppState>>) {
        (
            "/menus".to_string(),
            Router::new()
                .route("/add", post(create_menu_handler))
                .route("/{id}", get(get_menu_by_id))
                .route("/all", get(get_all_menus)),
            // 可添加更多菜单路由
        )
    }
}

struct RoleRouteRegistrar;
impl RouteRegistrar for RoleRouteRegistrar {
    fn register_routes() -> (String, Router<Arc<AppState>>) {
        (
            "/roles".to_string(),
            Router::new().route("/add", post(create_role_handler)),
            // .route("/{id}", get(get_role_handler))
            // .route("/{id}", put(update_role_handler))
            // .route("/{id}", delete(delete_role_handler))
            // .route("/list", get(get_all_role_handler)),
        )
    }
}

/// 创建最终路由（通用入口）
pub fn create_router(app_state: AppState) -> Router {
    let shared_state = Arc::new(app_state);
    let auth_middleware = AuthMiddleware::new(shared_state.config.auth.clone());
    // 创建公开路由（不需要认证）
    let public_routes = Router::new()
        .route("/health", get(health_check_handler))
        .route("/users/login", post(login_handler))
        .route("/users/add", post(create_user_handler))
        .with_state(shared_state.clone());

    // 创建受保护的路由（需要认证）
    let protected_routes = Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/{id}", get(get_user_handler))
                .route("/{id}", put(update_user_handler))
                .route("/{id}", delete(delete_user_handler))
                .route("/list", get(get_all_user_handler))
                .route("/logout", get(logout_handler))
            ,
        )
        .nest(
            "/menus",
            Router::new()
                .route("/add", post(create_menu_handler))
                .route("/{id}", get(get_menu_by_id))
                .route("/all", get(get_all_menus)),
        )
        .nest(
            "/roles",
            Router::new().route("/add", post(create_role_handler)),
        )
        .layer(from_fn_with_state(
            shared_state.clone(),
            move |request, next| {
                let auth_middleware = auth_middleware.clone();
                async move {
                    auth_middleware.auth_middleware(request, next).await
                }
            },
        ))
        .with_state(shared_state.clone());

    // 合并所有路由
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(
            tower::ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(CompressionLayer::new()),
        )
}
