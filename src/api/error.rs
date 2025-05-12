use thiserror::Error;

/// API error types
#[derive(Error, Debug)]
pub enum ApiError {
    /// Error during API client creation
    #[error("Failed to create API client: {0}")]
    ClientCreationError(String),

    /// Error during request execution
    #[error("Request error: {0}")]
    RequestError(String),

    /// Error parsing API response
    #[error("Failed to parse API response: {0}")]
    ResponseParseError(String),

    /// Resource not found (HTTP 404)
    #[error("Resource not found")]
    ResourceNotFound,

    /// Authentication error (HTTP 401)
    #[error("Authentication required")]
    Unauthorized,

    /// Authorization error (HTTP 403)
    #[error("Access forbidden")]
    Forbidden,

    /// Rate limit exceeded (HTTP 429)
    #[error("API rate limit exceeded")]
    RateLimitExceeded,

    /// Server error with status code and message
    #[error("Server error {0}: {1}")]
    ServerError(u16, String),

    /// Unsupported HTTP method
    #[error("Unsupported HTTP method")]
    UnsupportedMethod,

    /// Maximum retry count exceeded
    #[error("Maximum retry count exceeded")]
    MaxRetriesExceeded,

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Timeout error
    #[error("Request timed out")]
    Timeout,

    /// Connection error
    #[error("Failed to connect: {0}")]
    ConnectionError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}