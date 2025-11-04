mod shared;
// 设计优势
// 清晰的职责分离: 每个模块都有明确的责任范围，便于维护和扩展
// 依赖方向控制: 上层模块不直接依赖下层实现，通过接口解耦
// 可测试性: 各层都可以独立进行单元测试
// 可替换性: 基础设施实现可以轻松替换而不影响核心业务逻辑
// 团队协作友好: 不同团队可以并行开发不同层的功能
// 这种结构遵循了Clean Architecture原则，能够很好地支持大型Web API项目的长期发展需求


use ai_block_chain::config::Config;
use ai_block_chain::api::handlers::startup::Application;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = Config::load()?;

    // 启动应用
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;

    Ok(())
}
