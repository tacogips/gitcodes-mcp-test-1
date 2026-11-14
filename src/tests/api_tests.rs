#[cfg(test)]
mod api_tests {
    use crate::api::{ApiClient, ApiError, ApiRequest, ApiResponse};
    use crate::Config;
    use mockito::{mock, server_url};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        message: String,
        status: String,
    }

    #[tokio::test]
    async fn test_api_client_get() {
        let mock_server = server_url();
        
        // Create a mock for GET /test
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message":"success","status":"ok"}"#)
            .create();
        
        // Create API client with mock server URL
        let config = Config {
            api_url: mock_server,
            api_key: None,
            timeout: std::time::Duration::from_secs(1),
            max_retries: 3,
        };
        
        let client = ApiClient::new(config).unwrap();
        
        // Make the request
        let response: TestResponse = client.get("test").await.unwrap();
        
        // Verify response
        assert_eq!(response.message, "success");
        assert_eq!(response.status, "ok");
    }

    #[tokio::test]
    async fn test_api_client_post() {
        let mock_server = server_url();
        
        // Create request body
        #[derive(Debug, Serialize, Deserialize)]
        struct TestRequest {
            name: String,
            value: i32,
        }
        
        let request = TestRequest {
            name: "test".to_string(),
            value: 42,
        };
        
        // Create a mock for POST /test
        let _m = mock("POST", "/test")
            .with_status(201)
            .with_header("content-type", "application/json")
            .match_header("content-type", "application/json")
            .match_body(r#"{"name":"test","value":42}"#)
            .with_body(r#"{"message":"created","status":"ok"}"#)
            .create();
        
        // Create API client with mock server URL
        let config = Config {
            api_url: mock_server,
            api_key: None,
            timeout: std::time::Duration::from_secs(1),
            max_retries: 3,
        };
        
        let client = ApiClient::new(config).unwrap();
        
        // Make the request
        let response: TestResponse = client.post("test", &request).await.unwrap();
        
        // Verify response
        assert_eq!(response.message, "created");
        assert_eq!(response.status, "ok");
    }

    #[tokio::test]
    async fn test_api_client_error_handling() {
        let mock_server = server_url();
        
        // Create a mock for GET /error that returns 404
        let _m = mock("GET", "/error")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error":"Resource not found"}"#)
            .create();
        
        // Create API client with mock server URL
        let config = Config {
            api_url: mock_server,
            api_key: None,
            timeout: std::time::Duration::from_secs(1),
            max_retries: 3,
        };
        
        let client = ApiClient::new(config).unwrap();
        
        // Make the request and expect a ResourceNotFound error
        let result: Result<TestResponse, ApiError> = client.get("error").await;
        
        assert!(matches!(result, Err(ApiError::ResourceNotFound)));
    }

    #[tokio::test]
    async fn test_api_request_builder() {
        // Test building a GET request
        let request = ApiRequest::<()>::get("resources")
            .with_header("X-Custom-Header", "value")
            .with_query_param("filter", "active");
        
        assert_eq!(request.method().as_str(), "GET");
        assert_eq!(request.path(), "resources");
        assert_eq!(request.headers().get("X-Custom-Header"), Some(&"value".to_string()));
        assert_eq!(request.query_params().get("filter"), Some(&"active".to_string()));
        assert!(request.body().is_none());
        
        // Test building a POST request with a body
        #[derive(Debug, Serialize)]
        struct TestBody {
            name: String,
        }
        
        let body = TestBody {
            name: "test".to_string(),
        };
        
        let request = ApiRequest::post("resources")
            .with_json_content_type()
            .with_body(body);
        
        assert_eq!(request.method().as_str(), "POST");
        assert_eq!(request.path(), "resources");
        assert_eq!(
            request.headers().get("content-type"),
            Some(&"application/json".to_string())
        );
        assert!(request.body().is_some());
    }

    #[tokio::test]
    async fn test_api_response_methods() {
        // Create a response
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "content-type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "x-ratelimit-remaining",
            reqwest::header::HeaderValue::from_static("99"),
        );
        
        let response = ApiResponse::new(
            reqwest::StatusCode::OK,
            headers,
            TestResponse {
                message: "success".to_string(),
                status: "ok".to_string(),
            },
        );
        
        // Test response methods
        assert!(response.is_success());
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        assert_eq!(response.content_type(), Some("application/json"));
        assert_eq!(response.rate_limit_remaining(), Some(99));
        assert_eq!(response.body().message, "success");
        assert_eq!(response.body().status, "ok");
    }
}