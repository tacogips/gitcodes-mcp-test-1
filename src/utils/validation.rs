use regex::Regex;
use thiserror::Error;
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Validation error types
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Required field is missing
    #[error("Required field missing: {0}")]
    RequiredFieldMissing(String),
    
    /// Field value is invalid
    #[error("Invalid field value: {0} - {1}")]
    InvalidFieldValue(String, String),
    
    /// Field length is invalid
    #[error("Invalid field length: {0} - {1}")]
    InvalidFieldLength(String, String),
    
    /// Field format is invalid
    #[error("Invalid field format: {0} - {1}")]
    InvalidFieldFormat(String, String),
    
    /// Field value is outside allowed range
    #[error("Field value out of range: {0} - {1}")]
    OutOfRange(String, String),
    
    /// Multiple validation errors
    #[error("Multiple validation errors: {0} errors")]
    MultipleErrors(Vec<ValidationError>),
}

/// Common validation patterns
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()
});

static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap()
});

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9_-]{3,20}$").unwrap()
});

/// Validation result type
pub type ValidationResult = Result<(), ValidationError>;

/// Validation trait for models
pub trait Validate {
    /// Validate the model
    fn validate(&self) -> ValidationResult;
}

/// Helper function to validate that a string is not empty
pub fn validate_not_empty(field: &str, field_name: &str) -> ValidationResult {
    if field.trim().is_empty() {
        Err(ValidationError::RequiredFieldMissing(field_name.to_string()))
    } else {
        Ok(())
    }
}

/// Helper function to validate string length
pub fn validate_length(
    field: &str,
    field_name: &str,
    min_length: usize,
    max_length: usize,
) -> ValidationResult {
    let length = field.len();
    
    if length < min_length || length > max_length {
        Err(ValidationError::InvalidFieldLength(
            field_name.to_string(),
            format!("Length must be between {} and {}", min_length, max_length),
        ))
    } else {
        Ok(())
    }
}

/// Helper function to validate a number is within a range
pub fn validate_range<T>(
    value: T,
    field_name: &str,
    min_value: T,
    max_value: T,
) -> ValidationResult
where
    T: PartialOrd + std::fmt::Display,
{
    if value < min_value || value > max_value {
        Err(ValidationError::OutOfRange(
            field_name.to_string(),
            format!("Value must be between {} and {}", min_value, max_value),
        ))
    } else {
        Ok(())
    }
}

/// Helper function to validate an email address
pub fn validate_email(email: &str, field_name: &str) -> ValidationResult {
    if !EMAIL_REGEX.is_match(email) {
        Err(ValidationError::InvalidFieldFormat(
            field_name.to_string(),
            "Invalid email format".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Helper function to validate a URL
pub fn validate_url(url: &str, field_name: &str) -> ValidationResult {
    if !URL_REGEX.is_match(url) {
        Err(ValidationError::InvalidFieldFormat(
            field_name.to_string(),
            "Invalid URL format".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Helper function to validate a username
pub fn validate_username(username: &str, field_name: &str) -> ValidationResult {
    if !USERNAME_REGEX.is_match(username) {
        Err(ValidationError::InvalidFieldFormat(
            field_name.to_string(),
            "Username must be 3-20 characters and contain only letters, numbers, underscores, and hyphens".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate multiple fields and collect all errors
pub fn validate_all(validations: Vec<ValidationResult>) -> ValidationResult {
    let errors: Vec<ValidationError> = validations
        .into_iter()
        .filter_map(|result| match result {
            Ok(()) => None,
            Err(err) => Some(err),
        })
        .collect();
    
    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(ValidationError::MultipleErrors(errors))
    }
}

/// Validate a collection of required fields in a hash map
pub fn validate_required_fields(
    data: &HashMap<String, String>,
    required_fields: &[&str],
) -> ValidationResult {
    let mut errors = Vec::new();
    
    for &field in required_fields {
        if !data.contains_key(field) || data.get(field).map_or(true, |v| v.is_empty()) {
            errors.push(ValidationError::RequiredFieldMissing(field.to_string()));
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(ValidationError::MultipleErrors(errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("value", "field").is_ok());
        assert!(validate_not_empty("", "field").is_err());
        assert!(validate_not_empty("  ", "field").is_err());
    }
    
    #[test]
    fn test_validate_length() {
        assert!(validate_length("abc", "field", 2, 5).is_ok());
        assert!(validate_length("abcdef", "field", 2, 5).is_err());
        assert!(validate_length("a", "field", 2, 5).is_err());
    }
    
    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, "field", 1, 10).is_ok());
        assert!(validate_range(0, "field", 1, 10).is_err());
        assert!(validate_range(11, "field", 1, 10).is_err());
    }
    
    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com", "email").is_ok());
        assert!(validate_email("user@example.co.uk", "email").is_ok());
        assert!(validate_email("user.name+tag@example.com", "email").is_ok());
        assert!(validate_email("user@", "email").is_err());
        assert!(validate_email("@example.com", "email").is_err());
        assert!(validate_email("user@example", "email").is_err());
        assert!(validate_email("userexample.com", "email").is_err());
    }
    
    #[test]
    fn test_validate_url() {
        assert!(validate_url("https://example.com", "url").is_ok());
        assert!(validate_url("http://example.com/path?query=value", "url").is_ok());
        assert!(validate_url("example.com", "url").is_err());
        assert!(validate_url("https:/example.com", "url").is_err());
    }
    
    #[test]
    fn test_validate_username() {
        assert!(validate_username("user123", "username").is_ok());
        assert!(validate_username("user_name", "username").is_ok());
        assert!(validate_username("user-name", "username").is_ok());
        assert!(validate_username("us", "username").is_err());
        assert!(validate_username("user name", "username").is_err());
        assert!(validate_username("user@name", "username").is_err());
        assert!(validate_username("username_that_is_way_too_long", "username").is_err());
    }
    
    #[test]
    fn test_validate_all() {
        let validations = vec![
            validate_not_empty("value", "field1"),
            validate_length("abc", "field2", 2, 5),
        ];
        assert!(validate_all(validations).is_ok());
        
        let validations = vec![
            validate_not_empty("value", "field1"),
            validate_length("abcdef", "field2", 2, 5),
        ];
        assert!(validate_all(validations).is_err());
        
        let validations = vec![
            validate_not_empty("", "field1"),
            validate_length("abcdef", "field2", 2, 5),
        ];
        
        if let Err(ValidationError::MultipleErrors(errors)) = validate_all(validations) {
            assert_eq!(errors.len(), 2);
        } else {
            panic!("Expected MultipleErrors");
        }
    }
}