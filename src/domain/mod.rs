// Domain 层 (domain/)
// 包含核心业务规则和领域知识：
// entities/: 核心业务实体，包含业务行为和状态
// repositories/: 仓储模式接口，抽象数据访问操作
// events/: 领域事件，支持事件驱动架构
// value_objects/: 不可变的值对象，表示领域中的概念

pub mod entities;
pub mod enums;
pub mod events;
pub mod repositories;
pub mod traits;
pub mod value_objects;

