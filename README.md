# Rust Web API DDD

A modern Web API project built with Rust, following the Domain-Driven Design (DDD) architectural pattern.


## Project Features

- 🚀 Built with Rust for high performance and memory safety
- 🏗️ Follows the Domain-Driven Design (DDD) architectural pattern
- 🌐 Provides RESTful APIs using the Axum web framework
- 🗃️ Supports MySQL database (with dual ORMs: SQLx and Diesel)
- 🔥 Integrates Redis for caching
- 🔐 JWT authentication and authorization mechanism
- 🧪 Architecture designed for easy unit testing
- 📦 Modular design for extensibility and maintainability


## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **ORM**: SQLx + Diesel
- **Database**: MySQL
- **Caching**: Redis
- **Authentication**: JWT + Argon2
- **Logging**: tracing + log
- **Configuration Management**: config + dotenvy
- **Serialization**: serde


## Project Architecture

This project adheres to the principles of Clean Architecture and adopts a layered architectural design.


## Feature Modules

### User Management
- User registration
- User login/logout
- User information query
- User information update
- User deletion

### Menu Management
- Menu creation
- Menu query

### Role Management
- Role creation


## Quick Start

### Environment Requirements

- Rust 1.70 or higher
- MySQL 5.7 or higher
- Redis 5.0 or higher


### Configuration

1. Copy the configuration file template:


## Development Guide

### Adding New Modules

1. Create domain entities in `domain/entities/`
2. Define repository interfaces in `domain/traits/`
3. Define DTOs in `application/dtos/`
4. Implement business logic in `application/services/`
5. Implement repositories in `infrastructure/repositories/`
6. Create request handlers in `api/handlers/`
7. Register routes in [/routes/routes.rs)


### Code Standards

- Follow Rust's official coding standards
- Format code with `cargo fmt`
- Check code quality with `cargo clippy`


## License

MIT License


## Contributions

Contributions are welcome via Issues and Pull Requests to improve this project.
