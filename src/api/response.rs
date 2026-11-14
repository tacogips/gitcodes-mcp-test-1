use reqwest::{StatusCode, header::HeaderMap};

/// API response structure with status, headers, and body
pub struct ApiResponse<T> {
    status: StatusCode,
    headers: HeaderMap,
    body: T,
}

impl<T> ApiResponse<T> {
    /// Create a new API response
    pub fn new(status: StatusCode, headers: HeaderMap, body: T) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Get the response status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the response headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get a reference to the response body
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Take ownership of the response body
    pub fn into_body(self) -> T {
        self.body
    }

    /// Check if the response was successful (2xx status code)
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Get the value of a specific header
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|v| v.to_str().ok())
    }

    /// Check if the response has a specific header
    pub fn has_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }

    /// Get the content type header value
    pub fn content_type(&self) -> Option<&str> {
        self.header("content-type")
    }

    /// Get the rate limit remaining from headers
    pub fn rate_limit_remaining(&self) -> Option<u32> {
        self.header("x-ratelimit-remaining")
            .and_then(|v| v.parse::<u32>().ok())
    }

    /// Get the rate limit reset timestamp from headers
    pub fn rate_limit_reset(&self) -> Option<u64> {
        self.header("x-ratelimit-reset")
            .and_then(|v| v.parse::<u64>().ok())
    }
}