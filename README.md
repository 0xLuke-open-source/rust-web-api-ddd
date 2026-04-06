# Rust Web API DDD

一个基于 Rust 的现代化 Web API 项目，采用领域驱动设计（DDD）架构模式构建。

## 项目特点

- 🚀 基于 Rust 构建，高性能、内存安全
- 🏗️ 遵循领域驱动设计（DDD）架构模式
- 🌐 使用 Axum Web 框架提供 RESTful API
- 🗃️ 支持 MySQL 数据库（SQLx 和 Diesel 双 ORM）
- 🔥 集成 Redis 缓存支持
- 🔐 JWT 身份验证和授权机制
- 🧪 单元测试友好的架构设计
- 📦 模块化设计，易于扩展和维护

## 技术栈

- **语言**: Rust
- **Web框架**: Axum
- **ORM**: SQLx + Diesel
- **数据库**: MySQL
- **缓存**: Redis
- **身份验证**: JWT + Argon2
- **日志**: tracing + log
- **配置管理**: config + dotenvy
- **序列化**: serde

## 项目架构

本项目遵循 Clean Architecture 原则，采用分层架构设计：

## 功能模块

### 用户管理
- 用户注册
- 用户登录/登出
- 用户信息查询
- 用户信息更新
- 用户删除

### 菜单管理
- 菜单创建
- 菜单查询

### 角色管理
- 角色创建

## 快速开始

### 环境要求

- Rust 1.70 或更高版本
- MySQL 5.7 或更高版本
- Redis 5.0 或更高版本

## 功能模块

### 用户管理
- 用户注册
- 用户登录/登出
- 用户信息查询
- 用户信息更新
- 用户删除

### 菜单管理
- 菜单创建
- 菜单查询

### 角色管理
- 角色创建

## 快速开始

### 环境要求

- Rust 1.70 或更高版本
- MySQL 5.7 或更高版本
- Redis 5.0 或更高版本

### 配置

1. 复制配置文件模板：


## 开发指南

### 添加新模块

1. 在 `domain/entities/` 中创建领域实体
2. 在 `domain/traits/` 中定义仓储接口
3. 在 `application/dtos/` 中定义 DTO
4. 在 `application/services/` 中实现业务逻辑
5. 在 `infrastructure/repositories/` 中实现仓储
6. 在 `api/handlers/` 中创建请求处理器
7. 在 [api/routes/routes.rs](file:///Users/asa/work/project/AIME/aime-chain/rust/rust-web-api-ddd/src/api/routes/routes.rs) 中注册路由

### 代码规范

- 遵循 Rust 官方编码规范
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目。
