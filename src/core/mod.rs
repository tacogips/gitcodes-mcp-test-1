//! Core business logic module
//!
//! Contains the main application logic and service implementations.

pub mod error;
pub mod service;
pub mod processor;

pub use error::CoreError;
pub use service::Service;

/// Application state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    /// Application is initializing
    Initializing,
    /// Application is running normally
    Running,
    /// Application is shutting down
    ShuttingDown,
    /// Application is in maintenance mode
    Maintenance,
    /// Application has encountered an error
    Error,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Initializing
    }
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            AppState::Initializing => "initializing",
            AppState::Running => "running",
            AppState::ShuttingDown => "shutting_down",
            AppState::Maintenance => "maintenance",
            AppState::Error => "error",
        };
        write!(f, "{}", state_str)
    }
}

/// Feature flag configuration
pub struct FeatureFlags {
    pub enable_advanced_search: bool,
    pub enable_caching: bool,
    pub enable_metrics: bool,
    pub enable_rate_limiting: bool,
    pub experimental_features: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enable_advanced_search: true,
            enable_caching: true,
            enable_metrics: true,
            enable_rate_limiting: true,
            experimental_features: false,
        }
    }
}

/// Get the current application state
pub fn get_app_state() -> AppState {
    // In a real application, this would check some global state
    AppState::Running
}

/// Check if a feature is enabled
pub fn is_feature_enabled(feature: &str) -> bool {
    let flags = FeatureFlags::default();
    
    match feature {
        "advanced_search" => flags.enable_advanced_search,
        "caching" => flags.enable_caching,
        "metrics" => flags.enable_metrics,
        "rate_limiting" => flags.enable_rate_limiting,
        "experimental" => flags.experimental_features,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_display() {
        assert_eq!(AppState::Initializing.to_string(), "initializing");
        assert_eq!(AppState::Running.to_string(), "running");
        assert_eq!(AppState::ShuttingDown.to_string(), "shutting_down");
        assert_eq!(AppState::Maintenance.to_string(), "maintenance");
        assert_eq!(AppState::Error.to_string(), "error");
    }

    #[test]
    fn test_feature_flags() {
        assert!(is_feature_enabled("advanced_search"));
        assert!(is_feature_enabled("caching"));
        assert!(is_feature_enabled("metrics"));
        assert!(is_feature_enabled("rate_limiting"));
        assert!(!is_feature_enabled("experimental"));
        assert!(!is_feature_enabled("nonexistent_feature"));
    }
}