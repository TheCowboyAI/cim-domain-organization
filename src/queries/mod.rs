//! Query types for the Organization domain

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::value_objects::{OrganizationType, OrganizationStatus};

/// Get organization by ID
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationById {
    /// The organization ID to look up
    pub organization_id: Uuid,
}

/// Get organization hierarchy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationHierarchy {
    /// The root organization ID to start from
    pub organization_id: Uuid,
    /// Maximum depth to traverse (None = unlimited)
    pub max_depth: Option<usize>,
}

/// Get organization members
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationMembers {
    /// The organization ID
    pub organization_id: Uuid,
    /// Filter by role (optional)
    pub role_filter: Option<String>,
    /// Include inactive members
    pub include_inactive: bool,
}

/// Get organizations by type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationsByType {
    /// The organization type to filter by
    pub org_type: OrganizationType,
    /// Include child organizations
    pub include_children: bool,
}

/// Get organizations by status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationsByStatus {
    /// The status to filter by
    pub status: OrganizationStatus,
}

/// Get organizations a person is a member of
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMemberOrganizations {
    /// The person ID
    pub person_id: Uuid,
    /// Include inactive memberships
    pub include_inactive: bool,
}

/// Get reporting structure for an organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetReportingStructure {
    /// The organization ID
    pub organization_id: Uuid,
    /// Starting person ID (optional, defaults to those with no manager)
    pub starting_person_id: Option<Uuid>,
    /// Maximum depth to traverse
    pub max_depth: Option<usize>,
}

/// Search organizations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchOrganizations {
    /// Search query (searches name)
    pub query: String,
    /// Filter by type (optional)
    pub org_type_filter: Option<OrganizationType>,
    /// Filter by status (optional)
    pub status_filter: Option<OrganizationStatus>,
    /// Maximum results
    pub limit: usize,
}

/// Get organization statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationStatistics {
    /// The organization ID
    pub organization_id: Uuid,
}

/// Get organizations by location
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrganizationsByLocation {
    /// The location ID
    pub location_id: Uuid,
    /// Include organizations where this is not the primary location
    pub include_non_primary: bool,
}

/// Get organization chart
#[derive(Debug, Clone)]
pub struct GetOrganizationChart {
    pub organization_id: Uuid,
    pub layout_type: Option<String>,
}

/// Get organization's direct reports count
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDirectReportsCount {
    /// The organization ID
    pub organization_id: Uuid,
    /// The manager's person ID
    pub manager_id: Uuid,
}

/// Get organization role distribution
#[derive(Debug, Clone)]
pub struct GetOrganizationRoleDistribution {
    pub organization_id: Uuid,
}

/// Get organization location distribution
#[derive(Debug, Clone)]
pub struct GetOrganizationLocationDistribution {
    pub organization_id: Uuid,
}

/// Get organization size distribution
#[derive(Debug, Clone)]
pub struct GetOrganizationSizeDistribution {
    pub organization_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_creation() {
        let query = GetOrganizationById {
            organization_id: Uuid::new_v4(),
        };
        assert!(!query.organization_id.is_nil());

        let search = SearchOrganizations {
            query: "Tech".to_string(),
            org_type_filter: Some(OrganizationType::Company),
            status_filter: Some(OrganizationStatus::Active),
            limit: 10,
        };
        assert_eq!(search.query, "Tech");
        assert_eq!(search.limit, 10);
    }
} 