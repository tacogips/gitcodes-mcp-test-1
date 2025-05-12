use thiserror::Error;

/// Core error types for business logic
#[derive(Error, Debug)]
pub enum CoreError {
    /// General error with message
    #[error("Core error: {0}")]
    General(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    /// Resource already exists
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Processing error
    #[error("Processing error: {0}")]
    Processing(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    /// External service error
    #[error("External service error: {0}")]
    ExternalService(String),
    
    /// Database error
    #[error("Database error: {0}")]
    Database(String),
    
    /// API error from API module
    #[error("API error: {0}")]
    Api(#[from] crate::api::ApiError),
}

/// Common error handling utilities
pub trait ErrorHandler {
    /// Handle and log an error
    fn handle_error(&self, error: &CoreError);
    
    /// Convert an error to a user-friendly message
    fn user_friendly_message(&self, error: &CoreError) -> String;
}

/// Default error handler implementation
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &CoreError) {
        // In a real application, this would log to a file or service
        log::error!("Error occurred: {}", error);
        
        // Additional handling based on error type
        match error {
            CoreError::ExternalService(msg) => {
                log::warn!("External service issue: {}", msg);
            }
            CoreError::Database(msg) => {
                log::error!("Database error requires attention: {}", msg);
            }
            _ => {}
        }
    }
    
    fn user_friendly_message(&self, error: &CoreError) -> String {
        match error {
            CoreError::Validation(_) => "The provided data is invalid. Please check your input and try again.".to_string(),
            CoreError::NotFound(_) => "The requested resource could not be found.".to_string(),
            CoreError::AlreadyExists(_) => "This resource already exists.".to_string(),
            CoreError::PermissionDenied(_) => "You don't have permission to perform this action.".to_string(),
            CoreError::ExternalService(_) => "An external service is currently unavailable. Please try again later.".to_string(),
            _ => "An error occurred. Our team has been notified.".to_string(),
        }
    }
}