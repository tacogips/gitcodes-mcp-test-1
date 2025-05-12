//! Utility module with helper functions
//!
//! This module provides various utility functions used throughout the application.

pub mod id;
pub mod logging;
pub mod validation;

use std::time::{Duration, Instant};
use std::future::Future;

/// Retry a future with exponential backoff
///
/// Executes the provided async function up to `max_retries` times,
/// with exponential backoff between retries.
///
/// # Arguments
///
/// * `operation` - The async function to retry
/// * `max_retries` - Maximum number of retry attempts
/// * `initial_delay` - Initial delay duration in milliseconds
///
/// # Returns
///
/// The result of the async function or the last error if all retries fail
pub async fn retry_with_backoff<T, E, F, Fut>(
    operation: F,
    max_retries: u32,
    initial_delay: u64,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut current_retry = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(error) => {
                if current_retry >= max_retries {
                    return Err(error);
                }

                // Exponential backoff
                tokio::time::sleep(Duration::from_millis(delay)).await;
                delay *= 2; // Exponential growth
                current_retry += 1;
            }
        }
    }
}

/// Measure the execution time of an async function
///
/// # Arguments
///
/// * `operation` - The async function to measure
///
/// # Returns
///
/// A tuple containing the result of the async function and the execution time
pub async fn measure_time<T, F, Fut>(operation: F) -> (T, Duration)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    let start = Instant::now();
    let result = operation().await;
    let duration = start.elapsed();

    (result, duration)
}

/// Truncate a string to a maximum length with an ellipsis
///
/// # Arguments
///
/// * `s` - The string to truncate
/// * `max_len` - The maximum length
///
/// # Returns
///
/// The truncated string
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

/// Check if a string represents a valid email address
///
/// This is a very basic check, not comprehensive email validation.
///
/// # Arguments
///
/// * `email` - The string to check
///
/// # Returns
///
/// `true` if the string is a valid email, `false` otherwise
pub fn is_valid_email(email: &str) -> bool {
    // Basic validation: contains @ and at least one dot after @
    if !email.contains('@') {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return false;
    }

    parts[1].contains('.')
}

/// Parse key-value pairs from a string
///
/// Parses a string in the format "key1=value1,key2=value2" into a HashMap.
///
/// # Arguments
///
/// * `s` - The string to parse
///
/// # Returns
///
/// A HashMap containing the parsed key-value pairs
pub fn parse_key_value_pairs(s: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();

    for pair in s.split(',') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello", 10), "Hello");
        assert_eq!(truncate_string("Hello, World!", 5), "He...");
        assert_eq!(truncate_string("Hello", 5), "Hello");
        assert_eq!(truncate_string("Hello", 3), "...");
        assert_eq!(truncate_string("Hello", 0), "...");
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("first.last@example.co.uk"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@example"));
        assert!(!is_valid_email("userexample.com"));
        assert!(!is_valid_email(""));
    }

    #[test]
    fn test_parse_key_value_pairs() {
        let pairs = parse_key_value_pairs("key1=value1,key2=value2");
        assert_eq!(pairs.get("key1"), Some(&"value1".to_string()));
        assert_eq!(pairs.get("key2"), Some(&"value2".to_string()));
        assert_eq!(pairs.get("key3"), None);

        let pairs = parse_key_value_pairs("key1=value1");
        assert_eq!(pairs.get("key1"), Some(&"value1".to_string()));
        assert_eq!(pairs.len(), 1);

        let pairs = parse_key_value_pairs("");
        assert_eq!(pairs.len(), 0);

        let pairs = parse_key_value_pairs("invalid");
        assert_eq!(pairs.len(), 0);
    }
}