// Application 层 (application/)
// 业务逻辑的核心层，协调各领域对象完成具体任务：
// services/: 实现业务用例，编排领域对象执行业务流程
// dtos/: 定义数据传输对象，用于在不同层之间传递数据
// interfaces/: 定义应用服务接口，实现依赖倒置原则


pub mod dtos;
mod interfaces;
pub mod services;