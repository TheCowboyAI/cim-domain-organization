//! Organization role value object

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::role_level::RoleLevel;

/// A role within an organization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationRole {
    /// Unique identifier for this role
    pub role_id: Uuid,
    /// Human-readable role identifier (e.g., "eng-manager", "ceo")
    pub role_code: String,
    /// Display title for the role
    pub title: String,
    /// Department or area this role belongs to
    pub department: Option<String>,
    /// Role level in the hierarchy
    pub level: RoleLevel,
    /// Permissions associated with this role
    pub permissions: HashSet<Permission>,
    /// Additional role attributes
    pub attributes: HashMap<String, String>,
}

/// Permissions that can be assigned to roles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Organization management
    CreateOrganization,
    UpdateOrganization,
    DeleteOrganization,
    ViewOrganization,
    
    // Member management
    AddMember,
    RemoveMember,
    UpdateMemberRole,
    ViewMembers,
    
    // Hierarchy management
    CreateSubUnit,
    RemoveSubUnit,
    ModifyHierarchy,
    
    // Budget and finance
    ViewBudget,
    ApproveBudget,
    ModifyBudget,
    
    // Reporting
    ViewReports,
    CreateReports,
    ExportData,
    
    // Custom permissions
    Custom(String),
}

impl OrganizationRole {
    /// Create a new organization role
    pub fn new(role_code: String, title: String, level: RoleLevel) -> Self {
        Self {
            role_id: Uuid::new_v4(),
            role_code,
            title,
            department: None,
            level,
            permissions: HashSet::new(),
            attributes: HashMap::new(),
        }
    }

    /// Create a role with specific ID (for testing or imports)
    pub fn with_id(role_id: Uuid, role_code: String, title: String, level: RoleLevel) -> Self {
        Self {
            role_id,
            role_code,
            title,
            department: None,
            level,
            permissions: HashSet::new(),
            attributes: HashMap::new(),
        }
    }

    /// Add a permission to this role
    pub fn add_permission(&mut self, permission: Permission) {
        self.permissions.insert(permission);
    }

    /// Remove a permission from this role
    pub fn remove_permission(&mut self, permission: &Permission) -> bool {
        self.permissions.remove(permission)
    }

    /// Check if this role has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    /// Set the department for this role
    pub fn set_department(&mut self, department: String) {
        self.department = Some(department);
    }

    /// Create a CEO role
    pub fn ceo() -> Self {
        let mut role = Self::new("CEO".to_string(), "Chief Executive Officer".to_string(), RoleLevel::Executive);
        // CEO has all permissions
        for permission in Self::all_permissions() {
            role.add_permission(permission);
        }
        role
    }

    /// Create a CTO role
    pub fn cto() -> Self {
        let mut role = Self::new("CTO".to_string(), "Chief Technology Officer".to_string(), RoleLevel::Executive);
        // CTO has most permissions except financial
        role.add_permission(Permission::ViewOrganization);
        role.add_permission(Permission::UpdateOrganization);
        role.add_permission(Permission::AddMember);
        role.add_permission(Permission::RemoveMember);
        role.add_permission(Permission::UpdateMemberRole);
        role.add_permission(Permission::ViewMembers);
        role.add_permission(Permission::CreateSubUnit);
        role.add_permission(Permission::RemoveSubUnit);
        role.add_permission(Permission::ModifyHierarchy);
        role.add_permission(Permission::ViewBudget);
        role.add_permission(Permission::ViewReports);
        role.add_permission(Permission::CreateReports);
        role.add_permission(Permission::ExportData);
        role
    }

    /// Create a VP Engineering role
    pub fn vp_engineering() -> Self {
        let mut role = Self::new("VP_ENG".to_string(), "Vice President of Engineering".to_string(), RoleLevel::VicePresident);
        role.add_permission(Permission::ViewOrganization);
        role.add_permission(Permission::AddMember);
        role.add_permission(Permission::RemoveMember);
        role.add_permission(Permission::UpdateMemberRole);
        role.add_permission(Permission::ViewMembers);
        role.add_permission(Permission::CreateSubUnit);
        role.add_permission(Permission::ViewBudget);
        role.add_permission(Permission::ViewReports);
        role.add_permission(Permission::CreateReports);
        role.add_permission(Permission::ExportData);
        role
    }

    /// Create an engineering manager role
    pub fn engineering_manager() -> Self {
        let mut role = Self::new("ENG_MGR".to_string(), "Engineering Manager".to_string(), RoleLevel::Manager);
        role.add_permission(Permission::ViewOrganization);
        role.add_permission(Permission::AddMember);
        role.add_permission(Permission::UpdateMemberRole);
        role.add_permission(Permission::ViewMembers);
        role.add_permission(Permission::ViewBudget);
        role.add_permission(Permission::ViewReports);
        role.add_permission(Permission::CreateReports);
        role
    }

    /// Create a software engineer role
    pub fn software_engineer() -> Self {
        let mut role = Self::new("SW_ENG".to_string(), "Software Engineer".to_string(), RoleLevel::Mid);
        role.add_permission(Permission::ViewOrganization);
        role.add_permission(Permission::ViewMembers);
        role.add_permission(Permission::ViewReports);
        role
    }

    /// Get all possible permissions
    fn all_permissions() -> HashSet<Permission> {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::CreateOrganization);
        permissions.insert(Permission::UpdateOrganization);
        permissions.insert(Permission::DeleteOrganization);
        permissions.insert(Permission::ViewOrganization);
        permissions.insert(Permission::AddMember);
        permissions.insert(Permission::RemoveMember);
        permissions.insert(Permission::UpdateMemberRole);
        permissions.insert(Permission::ViewMembers);
        permissions.insert(Permission::CreateSubUnit);
        permissions.insert(Permission::RemoveSubUnit);
        permissions.insert(Permission::ModifyHierarchy);
        permissions.insert(Permission::ViewBudget);
        permissions.insert(Permission::ApproveBudget);
        permissions.insert(Permission::ModifyBudget);
        permissions.insert(Permission::ViewReports);
        permissions.insert(Permission::CreateReports);
        permissions.insert(Permission::ExportData);
        permissions
    }
}

/// Member assignment in an organization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationMember {
    /// Person ID
    pub person_id: Uuid,
    /// Organization ID
    pub organization_id: Uuid,
    /// Role within the organization
    pub role: OrganizationRole,
    /// When they joined
    pub joined_at: chrono::DateTime<chrono::Utc>,
    /// When their role ends (if temporary)
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Reports to (manager/supervisor person ID)
    pub reports_to: Option<Uuid>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl OrganizationMember {
    /// Create a new organization member
    pub fn new(person_id: Uuid, organization_id: Uuid, role: OrganizationRole) -> Self {
        Self {
            person_id,
            organization_id,
            role,
            joined_at: chrono::Utc::now(),
            ends_at: None,
            reports_to: None,
            metadata: HashMap::new(),
        }
    }

    /// Check if the member is currently active
    pub fn is_active(&self) -> bool {
        match self.ends_at {
            Some(end_date) => chrono::Utc::now() < end_date,
            None => true,
        }
    }

    /// Set the reporting relationship
    pub fn set_reports_to(&mut self, manager_id: Uuid) {
        self.reports_to = Some(manager_id);
    }

    /// Set an end date for this role assignment
    pub fn set_end_date(&mut self, end_date: chrono::DateTime<chrono::Utc>) {
        self.ends_at = Some(end_date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_creation() {
        let role = OrganizationRole::new(
            "test-role".to_string(),
            "Test Role".to_string(),
            RoleLevel::Mid,
        );
        
        assert_eq!(role.role_code, "test-role");
        assert_eq!(role.title, "Test Role");
        assert_eq!(role.level, RoleLevel::Mid);
        assert!(role.permissions.is_empty());
    }

    #[test]
    fn test_predefined_roles() {
        let ceo = OrganizationRole::ceo();
        assert_eq!(ceo.level, RoleLevel::Executive);
        assert!(ceo.has_permission(&Permission::CreateOrganization));
        assert!(ceo.has_permission(&Permission::ApproveBudget));

        let engineer = OrganizationRole::software_engineer();
        assert_eq!(engineer.level, RoleLevel::Mid);
        assert!(engineer.has_permission(&Permission::ViewOrganization));
        assert!(!engineer.has_permission(&Permission::ApproveBudget));
    }

    #[test]
    fn test_member_active_status() {
        let role = OrganizationRole::software_engineer();
        let mut member = OrganizationMember::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            role,
        );
        
        assert!(member.is_active());
        
        // Set end date in the past
        member.set_end_date(chrono::Utc::now() - chrono::Duration::days(1));
        assert!(!member.is_active());
        
        // Set end date in the future
        member.set_end_date(chrono::Utc::now() + chrono::Duration::days(30));
        assert!(member.is_active());
    }
} 