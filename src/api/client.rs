use crate::Config;
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use super::error::ApiError;
use super::request::ApiRequest;
use super::response::ApiResponse;

/// API client for making requests to external services
pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    /// Create a new API client with the given configuration
    pub fn new(config: Config) -> Result<Self, ApiError> {
        let client = ClientBuilder::new()
            .timeout(config.timeout)
            .build()
            .map_err(|e| ApiError::ClientCreationError(e.to_string()))?;

        Ok(Self { client, config })
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Update the API URL
    pub fn set_api_url(&mut self, api_url: String) {
        self.config.api_url = api_url;
    }

    /// Set the API key
    pub fn set_api_key(&mut self, api_key: Option<String>) {
        self.config.api_key = api_key;
    }

    /// Execute a GET request
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.config.api_url, endpoint);
        
        let mut request = self.client.get(&url);
        
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| ApiError::RequestError(e.to_string()))?;
            
        Self::process_response(response).await
    }

    /// Execute a POST request with a JSON body
    pub async fn post<T, R>(&self, endpoint: &str, body: &R) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        R: Serialize,
    {
        let url = format!("{}/{}", self.config.api_url, endpoint);
        
        let mut request = self.client.post(&url);
        
        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request
            .json(body)
            .send()
            .await
            .map_err(|e| ApiError::RequestError(e.to_string()))?;
            
        Self::process_response(response).await
    }

    /// Execute a custom API request
    pub async fn execute<T, R>(&self, request: ApiRequest<R>) -> Result<ApiResponse<T>, ApiError>
    where
        T: DeserializeOwned,
        R: Serialize,
    {
        // Build the full URL
        let url = match request.path().starts_with("http") {
            true => request.path().to_string(),
            false => format!("{}/{}", self.config.api_url, request.path()),
        };

        // Create the HTTP request based on the method
        let req_builder = match request.method() {
            reqwest::Method::GET => self.client.get(&url),
            reqwest::Method::POST => self.client.post(&url),
            reqwest::Method::PUT => self.client.put(&url),
            reqwest::Method::DELETE => self.client.delete(&url),
            reqwest::Method::PATCH => self.client.patch(&url),
            _ => return Err(ApiError::UnsupportedMethod),
        };

        // Add the API key if present
        let mut req_builder = if let Some(api_key) = &self.config.api_key {
            req_builder.header("Authorization", format!("Bearer {}", api_key))
        } else {
            req_builder
        };

        // Add headers
        for (key, value) in request.headers() {
            req_builder = req_builder.header(key, value);
        }

        // Add query parameters
        req_builder = req_builder.query(&request.query_params());

        // Add body if present
        let response = if let Some(body) = request.body() {
            req_builder.json(body)
        } else {
            req_builder
        }
        .send()
        .await
        .map_err(|e| ApiError::RequestError(e.to_string()))?;

        // Process the response
        let status = response.status();
        let headers = response.headers().clone();

        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let body = response
                    .json::<T>()
                    .await
                    .map_err(|e| ApiError::ResponseParseError(e.to_string()))?;

                Ok(ApiResponse::new(status, headers, body))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ResourceNotFound),
            StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden),
            StatusCode::TOO_MANY_REQUESTS => Err(ApiError::RateLimitExceeded),
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());

                Err(ApiError::ServerError(status.as_u16(), error_text))
            }
        }
    }

    // Helper method to process API responses
    async fn process_response<T>(response: reqwest::Response) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        
        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let body = response
                    .json::<T>()
                    .await
                    .map_err(|e| ApiError::ResponseParseError(e.to_string()))?;
                Ok(body)
            }
            StatusCode::NOT_FOUND => Err(ApiError::ResourceNotFound),
            StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized),
            StatusCode::FORBIDDEN => Err(ApiError::Forbidden),
            StatusCode::TOO_MANY_REQUESTS => Err(ApiError::RateLimitExceeded),
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                    
                Err(ApiError::ServerError(status.as_u16(), error_text))
            }
        }
    }
}