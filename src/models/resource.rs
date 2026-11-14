use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Resource type enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    /// Document resource (for text content)
    Document,
    /// User resource (for user profiles)
    User,
    /// Project resource (for project metadata)
    Project,
    /// Settings resource (for configuration)
    Settings,
    /// Media resource (for images, videos, etc.)
    Media,
    /// Any resource type (wildcard for processors)
    Any,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Document => write!(f, "document"),
            ResourceType::User => write!(f, "user"),
            ResourceType::Project => write!(f, "project"),
            ResourceType::Settings => write!(f, "settings"),
            ResourceType::Media => write!(f, "media"),
            ResourceType::Any => write!(f, "any"),
        }
    }
}

/// Resource data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceData {
    /// Resource name
    pub name: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource description
    pub description: Option<String>,
    /// Resource data fields (key-value pairs)
    pub data: HashMap<String, String>,
    /// Resource metadata
    pub metadata: HashMap<String, String>,
}

impl ResourceData {
    /// Create a new resource data instance
    pub fn new(name: &str, resource_type: ResourceType) -> Self {
        Self {
            name: name.to_string(),
            resource_type,
            description: None,
            data: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add a data field
    pub fn with_data(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Add a metadata field
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

/// Complete resource model including ID and timestamps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Unique resource ID
    pub id: String,
    /// Resource data
    pub data: ResourceData,
    /// Creation timestamp (ISO 8601)
    pub created_at: String,
    /// Last update timestamp (ISO 8601)
    pub updated_at: String,
    /// Resource owner (user ID)
    pub owner_id: Option<String>,
}

impl Resource {
    /// Create a new resource with the given ID and data
    pub fn new(id: &str, data: ResourceData) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        
        Self {
            id: id.to_string(),
            data,
            created_at: now.clone(),
            updated_at: now,
            owner_id: None,
        }
    }
    
    /// Set the resource owner
    pub fn with_owner(mut self, owner_id: &str) -> Self {
        self.owner_id = Some(owner_id.to_string());
        self
    }
    
    /// Update the resource's updated_at timestamp to now
    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    /// Check if the resource is owned by the given user
    pub fn is_owned_by(&self, user_id: &str) -> bool {
        match &self.owner_id {
            Some(owner) => owner == user_id,
            None => false,
        }
    }
}