use crate::models::{Resource, ResourceType};
use super::error::CoreError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Trait for resource processors
pub trait ResourceProcessor: Send + Sync {
    /// Process a resource and potentially modify it
    fn process(&self, resource: &mut Resource) -> Result<(), CoreError>;
    
    /// Check if this processor can handle the given resource type
    fn can_handle(&self, resource_type: &ResourceType) -> bool;
}

/// Registry for resource processors
pub struct ProcessorRegistry {
    processors: Arc<Mutex<HashMap<ResourceType, Vec<Box<dyn ResourceProcessor>>>>>,
}

impl ProcessorRegistry {
    /// Create a new empty processor registry
    pub fn new() -> Self {
        Self {
            processors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Register a processor for a specific resource type
    pub async fn register(&self, resource_type: ResourceType, processor: Box<dyn ResourceProcessor>) {
        let mut processors = self.processors.lock().await;
        
        processors
            .entry(resource_type)
            .or_insert_with(Vec::new)
            .push(processor);
    }
    
    /// Process a resource through all applicable processors
    pub async fn process(&self, resource: &mut Resource) -> Result<(), CoreError> {
        let processors = self.processors.lock().await;
        
        if let Some(type_processors) = processors.get(&resource.data.resource_type) {
            for processor in type_processors {
                processor.process(resource)?;
            }
        }
        
        // Process with wildcard processors (those that handle any type)
        if let Some(wildcard_processors) = processors.get(&ResourceType::Any) {
            for processor in wildcard_processors {
                processor.process(resource)?;
            }
        }
        
        Ok(())
    }
}

impl Default for ProcessorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Document processor for handling document resources
pub struct DocumentProcessor;

impl ResourceProcessor for DocumentProcessor {
    fn process(&self, resource: &mut Resource) -> Result<(), CoreError> {
        if let Some(content) = resource.data.data.get_mut("content") {
            // Example processing: ensure content is not empty
            if content.is_empty() {
                return Err(CoreError::Validation("Document content cannot be empty".to_string()));
            }
            
            // Example processing: trim content
            *content = content.trim().to_string();
        }
        
        Ok(())
    }
    
    fn can_handle(&self, resource_type: &ResourceType) -> bool {
        matches!(resource_type, ResourceType::Document)
    }
}

/// User processor for handling user resources
pub struct UserProcessor;

impl ResourceProcessor for UserProcessor {
    fn process(&self, resource: &mut Resource) -> Result<(), CoreError> {
        if let Some(email) = resource.data.data.get("email") {
            // Example validation: basic email format check
            if !email.contains('@') || !email.contains('.') {
                return Err(CoreError::Validation("Invalid email format".to_string()));
            }
        }
        
        // Example: ensure users have a created_at timestamp
        if !resource.data.data.contains_key("created_at") {
            let now = chrono::Utc::now().to_rfc3339();
            resource.data.data.insert("created_at".to_string(), now);
        }
        
        Ok(())
    }
    
    fn can_handle(&self, resource_type: &ResourceType) -> bool {
        matches!(resource_type, ResourceType::User)
    }
}

/// Audit logger processor for all resource types
pub struct AuditLogProcessor;

impl ResourceProcessor for AuditLogProcessor {
    fn process(&self, resource: &mut Resource) -> Result<(), CoreError> {
        // Example: add a last_modified timestamp to all resources
        let now = chrono::Utc::now().to_rfc3339();
        resource.data.data.insert("last_modified".to_string(), now);
        
        // In a real app, this would log the modification to an audit log
        log::info!(
            "Resource modified: id={}, type={:?}, name={}",
            resource.id,
            resource.data.resource_type,
            resource.data.name
        );
        
        Ok(())
    }
    
    fn can_handle(&self, _resource_type: &ResourceType) -> bool {
        // This processor handles all resource types
        true
    }
}