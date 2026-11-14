//! API module for handling external service communication
//!
//! This module provides functionality for interacting with external APIs.

pub mod client;
pub mod error;
pub mod request;
pub mod response;

pub use client::ApiClient;
pub use error::ApiError;

/// API version used for requests
pub const API_VERSION: &str = "v1";

/// Default timeout for API requests in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// API request rate limit (requests per minute)
pub const RATE_LIMIT: u32 = 100;

/// Helper function to build API URL paths
pub fn build_api_path(base_url: &str, resource: &str) -> String {
    format!("{}/api/{}/{}", base_url.trim_end_matches('/'), API_VERSION, resource)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_api_path() {
        // Test with trailing slash in base URL
        let path1 = build_api_path("https://api.example.com/", "users");
        assert_eq!(path1, "https://api.example.com/api/v1/users");
        
        // Test without trailing slash in base URL
        let path2 = build_api_path("https://api.example.com", "users");
        assert_eq!(path2, "https://api.example.com/api/v1/users");
    }
}