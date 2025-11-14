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
