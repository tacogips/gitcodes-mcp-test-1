use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// User role enum
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    /// Administrator role with full access
    Admin,
    /// Manager role with elevated access
    Manager,
    /// Standard user role
    User,
    /// Read-only user role
    ReadOnly,
    /// Guest role with minimal access
    Guest,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Manager => write!(f, "manager"),
            UserRole::User => write!(f, "user"),
            UserRole::ReadOnly => write!(f, "readonly"),
            UserRole::Guest => write!(f, "guest"),
        }
    }
}

/// Permission type for access control
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    /// Permission to create resources
    CreateResource,
    /// Permission to read resources
    ReadResource,
    /// Permission to update resources
    UpdateResource,
    /// Permission to delete resources
    DeleteResource,
    /// Permission to manage users
    ManageUsers,
    /// Permission to manage settings
    ManageSettings,
    /// Permission to view reports
    ViewReports,
    /// Permission to export data
    ExportData,
    /// Permission to import data
    ImportData,
    /// Custom permission with a name
    Custom(String),
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::CreateResource => write!(f, "create_resource"),
            Permission::ReadResource => write!(f, "read_resource"),
            Permission::UpdateResource => write!(f, "update_resource"),
            Permission::DeleteResource => write!(f, "delete_resource"),
            Permission::ManageUsers => write!(f, "manage_users"),
            Permission::ManageSettings => write!(f, "manage_settings"),
            Permission::ViewReports => write!(f, "view_reports"),
            Permission::ExportData => write!(f, "export_data"),
            Permission::ImportData => write!(f, "import_data"),
            Permission::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user ID
    pub id: String,
    /// User email address
    pub email: String,
    /// Display name
    pub name: String,
    /// User role
    pub role: UserRole,
    /// Additional permissions
    pub permissions: HashSet<Permission>,
    /// Account enabled status
    pub enabled: bool,
    /// Email verification status
    pub email_verified: bool,
    /// Creation timestamp (ISO 8601)
    pub created_at: String,
    /// Last login timestamp (ISO 8601)
    pub last_login: Option<String>,
}

impl User {
    /// Create a new user with the given ID, email, and name
    pub fn new(id: &str, email: &str, name: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        
        Self {
            id: id.to_string(),
            email: email.to_string(),
            name: name.to_string(),
            role: UserRole::User,
            permissions: HashSet::new(),
            enabled: true,
            email_verified: false,
            created_at: now,
            last_login: None,
        }
    }
    
    /// Set the user role
    pub fn with_role(mut self, role: UserRole) -> Self {
        self.role = role;
        self
    }
    
    /// Add a permission
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }
    
    /// Set the email verification status
    pub fn with_email_verified(mut self, verified: bool) -> Self {
        self.email_verified = verified;
        self
    }
    
    /// Record a user login
    pub fn record_login(&mut self) {
        self.last_login = Some(chrono::Utc::now().to_rfc3339());
    }
    
    /// Check if the user has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        // Admins have all permissions
        if self.role == UserRole::Admin {
            return true;
        }
        
        // Check explicit permissions
        self.permissions.contains(permission)
    }
    
    /// Get all permissions for this user, including role-based permissions
    pub fn all_permissions(&self) -> HashSet<Permission> {
        let mut all_perms = self.permissions.clone();
        
        // Add role-based permissions
        match self.role {
            UserRole::Admin => {
                // Admins have all permissions
                all_perms.insert(Permission::CreateResource);
                all_perms.insert(Permission::ReadResource);
                all_perms.insert(Permission::UpdateResource);
                all_perms.insert(Permission::DeleteResource);
                all_perms.insert(Permission::ManageUsers);
                all_perms.insert(Permission::ManageSettings);
                all_perms.insert(Permission::ViewReports);
                all_perms.insert(Permission::ExportData);
                all_perms.insert(Permission::ImportData);
            }
            UserRole::Manager => {
                // Managers have most permissions, but not user management
                all_perms.insert(Permission::CreateResource);
                all_perms.insert(Permission::ReadResource);
                all_perms.insert(Permission::UpdateResource);
                all_perms.insert(Permission::DeleteResource);
                all_perms.insert(Permission::ViewReports);
                all_perms.insert(Permission::ExportData);
                all_perms.insert(Permission::ImportData);
            }
            UserRole::User => {
                // Standard users have basic permissions
                all_perms.insert(Permission::CreateResource);
                all_perms.insert(Permission::ReadResource);
                all_perms.insert(Permission::UpdateResource);
                all_perms.insert(Permission::DeleteResource);
            }
            UserRole::ReadOnly => {
                // Read-only users can only read
                all_perms.insert(Permission::ReadResource);
            }
            UserRole::Guest => {
                // Guests can only read
                all_perms.insert(Permission::ReadResource);
            }
        }
        
        all_perms
    }
}