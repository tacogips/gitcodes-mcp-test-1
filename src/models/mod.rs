//! Models module for data structures
//!
//! Contains data models and persistence functionality.

pub mod resource;
pub mod user;
pub mod persistence;

pub use resource::{Resource, ResourceData, ResourceType};
pub use user::{User, UserRole, Permission};

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub connection_timeout: std::time::Duration,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "password".to_string(),
            database: "app_db".to_string(),
            max_connections: 10,
            connection_timeout: std::time::Duration::from_secs(5),
        }
    }
}

/// Database connection string builder
pub fn build_connection_string(config: &DbConfig) -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_connection_string() {
        let config = DbConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "test_user".to_string(),
            password: "test_pass".to_string(),
            database: "test_db".to_string(),
            max_connections: 5,
            connection_timeout: std::time::Duration::from_secs(3),
        };

        let connection_string = build_connection_string(&config);
        assert_eq!(
            connection_string,
            "postgres://test_user:test_pass@localhost:5432/test_db"
        );
    }
}