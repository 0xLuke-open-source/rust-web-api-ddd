use config::{ConfigError, Environment, File};
use config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub app: AppConfig,
    pub logging: LoggingConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub hash_salt: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub exclude_paths: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into());

        config::Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?
            .try_deserialize()
    }
}