//! Rust Project Example
//! 
//! A library for demonstrating a multi-module Rust project.
//! This crate provides various functionalities for testing GitHub API integration.

pub mod api;
pub mod core;
pub mod models;
pub mod utils;

/// Current library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the application
/// 
/// Sets up logging and other global state.
pub fn initialize() {
    env_logger::init();
    log::info!("Application initialized, version: {}", VERSION);
}

/// Application-wide configuration
pub struct Config {
    pub api_url: String,
    pub api_key: Option<String>,
    pub timeout: std::time::Duration,
    pub max_retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "https://api.example.com".to_string(),
            api_key: None,
            timeout: std::time::Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

/// Create a new configuration with custom settings
pub fn create_config(api_url: Option<String>, api_key: Option<String>) -> Config {
    let mut config = Config::default();
    
    if let Some(url) = api_url {
        config.api_url = url;
    }
    
    if let Some(key) = api_key {
        config.api_key = Some(key);
    }
    
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.api_url, "https://api.example.com");
        assert_eq!(config.api_key, None);
        assert_eq!(config.timeout, std::time::Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_create_config() {
        let config = create_config(
            Some("https://custom-api.example.com".to_string()), 
            Some("test-key".to_string())
        );
        
        assert_eq!(config.api_url, "https://custom-api.example.com");
        assert_eq!(config.api_key, Some("test-key".to_string()));
    }
}