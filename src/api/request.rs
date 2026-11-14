use reqwest::{Method, header};
use serde::Serialize;
use std::collections::HashMap;

/// API request structure for building various requests
pub struct ApiRequest<T> {
    method: Method,
    path: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Option<T>,
}

impl<T> ApiRequest<T>
where
    T: Serialize,
{
    /// Create a new API request
    pub fn new(method: Method, path: &str) -> Self {
        Self {
            method,
            path: path.to_string(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
        }
    }

    /// Create a GET request
    pub fn get(path: &str) -> Self {
        Self::new(Method::GET, path)
    }

    /// Create a POST request
    pub fn post(path: &str) -> Self {
        Self::new(Method::POST, path)
    }

    /// Create a PUT request
    pub fn put(path: &str) -> Self {
        Self::new(Method::PUT, path)
    }

    /// Create a DELETE request
    pub fn delete(path: &str) -> Self {
        Self::new(Method::DELETE, path)
    }

    /// Create a PATCH request
    pub fn patch(path: &str) -> Self {
        Self::new(Method::PATCH, path)
    }

    /// Get the request method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get the request path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get request headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get query parameters
    pub fn query_params(&self) -> &HashMap<String, String> {
        &self.query_params
    }

    /// Get request body
    pub fn body(&self) -> Option<&T> {
        self.body.as_ref()
    }

    /// Add a header to the request
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Add a JSON content type header
    pub fn with_json_content_type(self) -> Self {
        self.with_header(
            header::CONTENT_TYPE.as_str(), 
            "application/json"
        )
    }

    /// Add a query parameter
    pub fn with_query_param(mut self, key: &str, value: &str) -> Self {
        self.query_params.insert(key.to_string(), value.to_string());
        self
    }

    /// Add multiple query parameters
    pub fn with_query_params(mut self, params: HashMap<String, String>) -> Self {
        self.query_params.extend(params);
        self
    }

    /// Set the request body
    pub fn with_body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }
}