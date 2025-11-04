// API 层 (api/)
// 负责处理与外部世界的交互，是系统的入口点：
// handlers/: 实现具体的HTTP请求处理逻辑，调用应用层服务完成业务操作
// middleware/: 提供跨切面功能，如身份验证、日志记录、请求限流等
// routes/: 定义API路由映射关系，将URL路径映射到相应的处理器
// validators/: 验证请求参数的有效性和安全性

pub mod handlers;
pub mod middleware;
pub mod routes;
mod validators;
mod repository;


pub use handlers::*;
pub use routes::routes::*;