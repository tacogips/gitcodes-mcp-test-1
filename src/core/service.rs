use crate::api::{ApiClient, ApiError};
use crate::models::{Resource, ResourceData, ResourceType};
use crate::Config;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::error::CoreError;

/// Generic service trait for resource operations
#[async_trait]
pub trait Service<T> {
    /// Create a new resource
    async fn create(&self, data: T) -> Result<T, CoreError>;
    
    /// Get a resource by ID
    async fn get(&self, id: &str) -> Result<T, CoreError>;
    
    /// Update an existing resource
    async fn update(&self, id: &str, data: T) -> Result<T, CoreError>;
    
    /// Delete a resource by ID
    async fn delete(&self, id: &str) -> Result<bool, CoreError>;
    
    /// List all resources with optional filtering
    async fn list(&self, limit: Option<usize>, filter: Option<&str>) -> Result<Vec<T>, CoreError>;
}

/// Primary implementation of the Service trait for Resource types
pub struct ResourceService {
    client: Arc<ApiClient>,
    cache: Arc<RwLock<ResourceCache>>,
}

/// Simple in-memory cache for resources
struct ResourceCache {
    resources: Vec<Resource>,
    last_updated: chrono::DateTime<chrono::Utc>,
}

impl ResourceCache {
    /// Create a new empty cache
    fn new() -> Self {
        Self {
            resources: Vec::new(),
            last_updated: chrono::Utc::now(),
        }
    }
    
    /// Check if the cache is stale (older than 5 minutes)
    fn is_stale(&self) -> bool {
        let now = chrono::Utc::now();
        let age = now - self.last_updated;
        age.num_minutes() > 5
    }
    
    /// Update the cache with new data
    fn update(&mut self, resources: Vec<Resource>) {
        self.resources = resources;
        self.last_updated = chrono::Utc::now();
    }
    
    /// Get a resource by ID from the cache
    fn get(&self, id: &str) -> Option<Resource> {
        self.resources.iter().find(|r| r.id == id).cloned()
    }
}

impl ResourceService {
    /// Create a new ResourceService with the given configuration
    pub fn new(config: Config) -> Result<Self, CoreError> {
        let client = ApiClient::new(config)
            .map_err(|e| CoreError::ExternalService(format!("Failed to create API client: {}", e)))?;
            
        Ok(Self {
            client: Arc::new(client),
            cache: Arc::new(RwLock::new(ResourceCache::new())),
        })
    }
    
    /// Create a new ResourceService with an existing API client
    pub fn with_client(client: Arc<ApiClient>) -> Self {
        Self {
            client,
            cache: Arc::new(RwLock::new(ResourceCache::new())),
        }
    }
    
    /// Get the API client
    pub fn client(&self) -> Arc<ApiClient> {
        self.client.clone()
    }
    
    /// Invalidate the cache, forcing a refresh on next fetch
    pub async fn invalidate_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.last_updated = chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(0, 0), 
            chrono::Utc
        );
    }
    
    /// Validate resource data before sending to the API
    fn validate(&self, data: &ResourceData) -> Result<(), CoreError> {
        if data.name.is_empty() {
            return Err(CoreError::Validation("Resource name cannot be empty".to_string()));
        }
        
        if data.name.len() > 100 {
            return Err(CoreError::Validation("Resource name too long (max 100 characters)".to_string()));
        }
        
        // Validate specific resource types
        match data.resource_type {
            ResourceType::Document => {
                // Documents should have content
                if data.data.get("content").is_none() {
                    return Err(CoreError::Validation("Document must have content".to_string()));
                }
            }
            ResourceType::User => {
                // Users should have an email
                if data.data.get("email").is_none() {
                    return Err(CoreError::Validation("User must have an email".to_string()));
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

#[async_trait]
impl Service<Resource> for ResourceService {
    async fn create(&self, resource: Resource) -> Result<Resource, CoreError> {
        // Validate the resource data
        self.validate(&resource.data)?;
        
        // Send the request to the API
        let result = self.client.post::<Resource, Resource>("resources", &resource)
            .await
            .map_err(|e| match e {
                ApiError::ResponseParseError(msg) => CoreError::Processing(msg),
                ApiError::Unauthorized => CoreError::PermissionDenied("Not authorized to create resources".to_string()),
                _ => CoreError::ExternalService(format!("API error: {}", e)),
            })?;
        
        // Invalidate the cache since we've modified data
        self.invalidate_cache().await;
        
        Ok(result)
    }
    
    async fn get(&self, id: &str) -> Result<Resource, CoreError> {
        // Check the cache first
        let cache = self.cache.read().await;
        
        if !cache.is_stale() {
            if let Some(resource) = cache.get(id) {
                return Ok(resource);
            }
        }
        drop(cache); // Release the read lock
        
        // Cache miss or stale, fetch from API
        let result = self.client.get::<Resource>(&format!("resources/{}", id))
            .await
            .map_err(|e| match e {
                ApiError::ResourceNotFound => CoreError::NotFound(format!("Resource not found: {}", id)),
                ApiError::Unauthorized => CoreError::PermissionDenied("Not authorized to access this resource".to_string()),
                _ => CoreError::ExternalService(format!("API error: {}", e)),
            })?;
        
        Ok(result)
    }
    
    async fn update(&self, id: &str, resource: Resource) -> Result<Resource, CoreError> {
        // Validate the resource data
        self.validate(&resource.data)?;
        
        // Ensure the ID in the path matches the ID in the resource
        if resource.id != id {
            return Err(CoreError::Validation("Resource ID mismatch".to_string()));
        }
        
        // Send the request to the API
        let result = self.client.post::<Resource, Resource>(&format!("resources/{}", id), &resource)
            .await
            .map_err(|e| match e {
                ApiError::ResourceNotFound => CoreError::NotFound(format!("Resource not found: {}", id)),
                ApiError::Unauthorized => CoreError::PermissionDenied("Not authorized to update this resource".to_string()),
                _ => CoreError::ExternalService(format!("API error: {}", e)),
            })?;
        
        // Invalidate the cache since we've modified data
        self.invalidate_cache().await;
        
        Ok(result)
    }
    
    async fn delete(&self, id: &str) -> Result<bool, CoreError> {
        // Send the request to the API
        let result: bool = self.client.get(&format!("resources/{}/delete", id))
            .await
            .map_err(|e| match e {
                ApiError::ResourceNotFound => CoreError::NotFound(format!("Resource not found: {}", id)),
                ApiError::Unauthorized => CoreError::PermissionDenied("Not authorized to delete this resource".to_string()),
                _ => CoreError::ExternalService(format!("API error: {}", e)),
            })?;
        
        // Invalidate the cache since we've modified data
        self.invalidate_cache().await;
        
        Ok(result)
    }
    
    async fn list(&self, limit: Option<usize>, filter: Option<&str>) -> Result<Vec<Resource>, CoreError> {
        // Check the cache first
        let cache = self.cache.read().await;
        
        if !cache.is_stale() {
            let resources = cache.resources.clone();
            drop(cache); // Release the read lock
            
            let filtered = match filter {
                Some(f) => resources.into_iter()
                    .filter(|r| r.data.name.contains(f))
                    .collect::<Vec<_>>(),
                None => resources,
            };
            
            let limited = match limit {
                Some(l) => filtered.into_iter().take(l).collect(),
                None => filtered,
            };
            
            return Ok(limited);
        }
        drop(cache); // Release the read lock
        
        // Cache is stale, fetch from API
        let mut endpoint = String::from("resources");
        
        // Add query parameters if needed
        let mut query_params = Vec::new();
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        if let Some(f) = filter {
            query_params.push(format!("filter={}", f));
        }
        
        if !query_params.is_empty() {
            endpoint = format!("{}?{}", endpoint, query_params.join("&"));
        }
        
        // Fetch from the API
        let result = self.client.get::<Vec<Resource>>(&endpoint)
            .await
            .map_err(|e| match e {
                ApiError::Unauthorized => CoreError::PermissionDenied("Not authorized to list resources".to_string()),
                _ => CoreError::ExternalService(format!("API error: {}", e)),
            })?;
        
        // Update the cache with the new data
        let mut cache = self.cache.write().await;
        cache.update(result.clone());
        
        Ok(result)
    }
}