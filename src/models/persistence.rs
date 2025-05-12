use crate::models::{Resource, User};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Repository trait for data persistence
#[async_trait]
pub trait Repository<T, ID> {
    /// Save an entity to the repository
    async fn save(&self, entity: T) -> Result<T, PersistenceError>;
    
    /// Find an entity by ID
    async fn find_by_id(&self, id: &ID) -> Result<Option<T>, PersistenceError>;
    
    /// Delete an entity by ID
    async fn delete(&self, id: &ID) -> Result<bool, PersistenceError>;
    
    /// Find all entities
    async fn find_all(&self) -> Result<Vec<T>, PersistenceError>;
    
    /// Count entities
    async fn count(&self) -> Result<usize, PersistenceError>;
}

/// Persistence errors
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    /// Query error
    #[error("Query error: {0}")]
    QueryError(String),
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Not found error
    #[error("Entity not found: {0}")]
    NotFoundError(String),
    
    /// Unique constraint violation
    #[error("Unique constraint violation: {0}")]
    UniqueConstraintViolation(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

/// In-memory repository implementation for Resource
pub struct InMemoryResourceRepository {
    resources: Arc<RwLock<HashMap<String, Resource>>>,
}

impl InMemoryResourceRepository {
    /// Create a new empty in-memory resource repository
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Repository<Resource, String> for InMemoryResourceRepository {
    async fn save(&self, resource: Resource) -> Result<Resource, PersistenceError> {
        let mut resources = self.resources.write().await;
        
        // Clone the resource before inserting it
        let resource_clone = resource.clone();
        resources.insert(resource.id.clone(), resource);
        
        Ok(resource_clone)
    }
    
    async fn find_by_id(&self, id: &String) -> Result<Option<Resource>, PersistenceError> {
        let resources = self.resources.read().await;
        Ok(resources.get(id).cloned())
    }
    
    async fn delete(&self, id: &String) -> Result<bool, PersistenceError> {
        let mut resources = self.resources.write().await;
        Ok(resources.remove(id).is_some())
    }
    
    async fn find_all(&self) -> Result<Vec<Resource>, PersistenceError> {
        let resources = self.resources.read().await;
        Ok(resources.values().cloned().collect())
    }
    
    async fn count(&self) -> Result<usize, PersistenceError> {
        let resources = self.resources.read().await;
        Ok(resources.len())
    }
}

impl Default for InMemoryResourceRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// In-memory repository implementation for User
pub struct InMemoryUserRepository {
    users: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    /// Create a new empty in-memory user repository
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Repository<User, String> for InMemoryUserRepository {
    async fn save(&self, user: User) -> Result<User, PersistenceError> {
        let mut users = self.users.write().await;
        
        // Clone the user before inserting it
        let user_clone = user.clone();
        users.insert(user.id.clone(), user);
        
        Ok(user_clone)
    }
    
    async fn find_by_id(&self, id: &String) -> Result<Option<User>, PersistenceError> {
        let users = self.users.read().await;
        Ok(users.get(id).cloned())
    }
    
    async fn delete(&self, id: &String) -> Result<bool, PersistenceError> {
        let mut users = self.users.write().await;
        Ok(users.remove(id).is_some())
    }
    
    async fn find_all(&self) -> Result<Vec<User>, PersistenceError> {
        let users = self.users.read().await;
        Ok(users.values().cloned().collect())
    }
    
    async fn count(&self) -> Result<usize, PersistenceError> {
        let users = self.users.read().await;
        Ok(users.len())
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating repositories
pub struct RepositoryFactory {
    resource_repository: Arc<dyn Repository<Resource, String> + Send + Sync>,
    user_repository: Arc<dyn Repository<User, String> + Send + Sync>,
}

impl RepositoryFactory {
    /// Create a new repository factory with in-memory repositories
    pub fn new_in_memory() -> Self {
        Self {
            resource_repository: Arc::new(InMemoryResourceRepository::new()),
            user_repository: Arc::new(InMemoryUserRepository::new()),
        }
    }
    
    /// Get the resource repository
    pub fn resource_repository(&self) -> Arc<dyn Repository<Resource, String> + Send + Sync> {
        self.resource_repository.clone()
    }
    
    /// Get the user repository
    pub fn user_repository(&self) -> Arc<dyn Repository<User, String> + Send + Sync> {
        self.user_repository.clone()
    }
}

impl Default for RepositoryFactory {
    fn default() -> Self {
        Self::new_in_memory()
    }
}