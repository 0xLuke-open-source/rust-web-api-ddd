pub mod config;
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod api;
pub mod shared;

pub use shared::api_response::ApiResponse;
// 重新导出常用类型
pub use domain::entities::{NewUser, UpdateUser, User};
pub use application::services::user_service::UserService;