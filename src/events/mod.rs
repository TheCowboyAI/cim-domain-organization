//! Domain events for the Organization domain

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_objects::{
    OrganizationType, OrganizationStatus, OrganizationRole, OrganizationMember,
};

/// Organization was created
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCreated {
    /// The unique identifier of the organization
    pub organization_id: Uuid,
    /// The name of the organization
    pub name: String,
    /// The type of organization
    pub org_type: OrganizationType,
    /// The parent organization ID if this is a sub-organization
    pub parent_id: Option<Uuid>,
    /// The primary location ID for this organization
    pub primary_location_id: Option<Uuid>,
    /// When the organization was created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Organization was updated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationUpdated {
    /// The organization that was updated
    pub organization_id: Uuid,
    /// New name if changed
    pub name: Option<String>,
    /// New primary location if changed
    pub primary_location_id: Option<Uuid>,
    /// When the update occurred
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Organization status changed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationStatusChanged {
    /// The organization whose status changed
    pub organization_id: Uuid,
    /// Previous status
    pub old_status: OrganizationStatus,
    /// New status
    pub new_status: OrganizationStatus,
    /// Reason for status change
    pub reason: Option<String>,
    /// When the change occurred
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

/// Member was added to organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberAdded {
    /// The organization receiving the new member
    pub organization_id: Uuid,
    /// The complete member information
    pub member: OrganizationMember,
    /// When the member was added
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Member was removed from organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberRemoved {
    /// The organization losing the member
    pub organization_id: Uuid,
    /// The person being removed
    pub person_id: Uuid,
    /// Reason for removal
    pub reason: Option<String>,
    /// When the member was removed
    pub removed_at: chrono::DateTime<chrono::Utc>,
}

/// Member role was updated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberRoleUpdated {
    /// The organization
    pub organization_id: Uuid,
    /// The member whose role changed
    pub person_id: Uuid,
    /// Previous role
    pub old_role: OrganizationRole,
    /// New role
    pub new_role: OrganizationRole,
    /// When the role was updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Reporting relationship changed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingRelationshipChanged {
    /// The organization
    pub organization_id: Uuid,
    /// The person whose manager changed
    pub person_id: Uuid,
    /// Previous manager (if any)
    pub old_manager_id: Option<Uuid>,
    /// New manager (if any)
    pub new_manager_id: Option<Uuid>,
    /// When the change occurred
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

/// Child organization was added
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildOrganizationAdded {
    /// Parent organization
    pub parent_id: Uuid,
    /// Child organization
    pub child_id: Uuid,
    /// When the relationship was established
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Child organization was removed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildOrganizationRemoved {
    /// Parent organization
    pub parent_id: Uuid,
    /// Child organization
    pub child_id: Uuid,
    /// When the relationship was removed
    pub removed_at: chrono::DateTime<chrono::Utc>,
}

/// Location was added to organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationAdded {
    /// The organization
    pub organization_id: Uuid,
    /// The location ID
    pub location_id: Uuid,
    /// Whether this is now the primary location
    pub is_primary: bool,
    /// When the location was added
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Location was removed from organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationRemoved {
    /// The organization
    pub organization_id: Uuid,
    /// The location ID
    pub location_id: Uuid,
    /// When the location was removed
    pub removed_at: chrono::DateTime<chrono::Utc>,
}

/// Primary location changed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryLocationChanged {
    /// The organization
    pub organization_id: Uuid,
    /// Previous primary location
    pub old_location_id: Option<Uuid>,
    /// New primary location
    pub new_location_id: Uuid,
    /// When the change occurred
    pub changed_at: chrono::DateTime<chrono::Utc>,
}

/// Organization was dissolved
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationDissolved {
    /// The organization that was dissolved
    pub organization_id: Uuid,
    /// Reason for dissolution
    pub reason: String,
    /// What happens to members
    pub member_disposition: MemberDisposition,
    /// When it was dissolved
    pub dissolved_at: chrono::DateTime<chrono::Utc>,
}

/// What happens to members when an organization is dissolved
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemberDisposition {
    /// Members are terminated
    Terminated,
    /// Members are transferred to another organization
    TransferredTo(Uuid),
    /// Members become independent contractors
    ConvertedToContractors,
    /// Other disposition
    Other(String),
}

/// Organization was merged
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMerged {
    /// The organization being merged (will be dissolved)
    pub source_organization_id: Uuid,
    /// The organization receiving the merge
    pub target_organization_id: Uuid,
    /// How members are handled
    pub member_disposition: MemberDisposition,
    /// When the merge occurred
    pub merged_at: chrono::DateTime<chrono::Utc>,
}

/// Organization was acquired
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAcquired {
    /// The organization being acquired
    pub acquired_organization_id: Uuid,
    /// The acquiring organization
    pub acquiring_organization_id: Uuid,
    /// Whether the acquired org maintains independence
    pub maintains_independence: bool,
    /// When the acquisition occurred
    pub acquired_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = OrganizationCreated {
            organization_id: Uuid::new_v4(),
            name: "Test Org".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
            created_at: chrono::Utc::now(),
        };

        assert_eq!(event.name, "Test Org");
        assert_eq!(event.org_type, OrganizationType::Company);
    }

    #[test]
    fn test_member_disposition() {
        let disposition = MemberDisposition::TransferredTo(Uuid::new_v4());
        match disposition {
            MemberDisposition::TransferredTo(org_id) => {
                assert!(!org_id.is_nil());
            }
            _ => panic!("Wrong disposition type"),
        }
    }
} 