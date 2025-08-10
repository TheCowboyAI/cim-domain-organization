//! Projections and read models for the Organization domain

pub mod views;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::value_objects::{OrganizationType, OrganizationStatus, OrganizationRole, RoleLevel, SizeCategory};

/// Organization view for queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationView {
    /// Organization's unique identifier
    pub organization_id: Uuid,
    /// Name of the organization
    pub name: String,
    /// Type of organization
    pub org_type: OrganizationType,
    /// Status of the organization
    pub status: OrganizationStatus,
    /// Parent organization ID
    pub parent_id: Option<Uuid>,
    /// Child organization IDs
    pub child_units: Vec<Uuid>,
    /// Member count
    pub member_count: usize,
    /// Size category based on member count
    pub size_category: SizeCategory,
}

impl OrganizationView {
    /// Calculate size category based on member count
    pub fn calculate_size_category(member_count: usize) -> SizeCategory {
        match member_count {
            0..=10 => SizeCategory::Small,
            11..=50 => SizeCategory::Medium,
            51..=200 => SizeCategory::Large,
            _ => SizeCategory::Enterprise,
        }
    }
    
    /// Update size category based on current member count
    pub fn update_size_category(&mut self) {
        self.size_category = Self::calculate_size_category(self.member_count);
    }
}

/// Hierarchical organization view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationHierarchyView {
    /// The organization at this level
    pub organization: OrganizationView,
    /// Child organizations
    pub children: Vec<OrganizationHierarchyView>,
}

/// Member view for queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberView {
    /// Person ID
    pub person_id: Uuid,
    /// Person's name
    pub person_name: String,
    /// Role in the organization
    pub role: OrganizationRole,
    /// When they joined
    pub joined_at: chrono::DateTime<chrono::Utc>,
    /// Reports to (manager ID)
    pub reports_to_id: Option<Uuid>,
    /// Reports to (manager name)
    pub reports_to_name: Option<String>,
    /// Number of direct reports
    pub direct_reports_count: usize,
    /// Is currently active
    pub is_active: bool,
}

/// View of a person's organization memberships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberOrganizationView {
    /// Organization ID
    pub organization_id: Uuid,
    /// Organization name
    pub organization_name: String,
    /// Organization type
    pub org_type: OrganizationType,
    /// Role in this organization
    pub role: OrganizationRole,
    /// Is primary organization
    pub is_primary: bool,
    /// Joined date
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

/// Reporting structure view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingStructureView {
    /// Organization ID
    pub organization_id: Uuid,
    /// Top-level members (those with no manager)
    pub root_members: Vec<ReportingNode>,
}

/// Node in the reporting structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportingNode {
    /// Person ID
    pub person_id: Uuid,
    /// Person's name
    pub person_name: String,
    /// Role
    pub role: OrganizationRole,
    /// Direct reports
    pub direct_reports: Vec<ReportingNode>,
}

/// Organization statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationStatistics {
    /// Organization ID
    pub organization_id: Uuid,
    /// Total member count
    pub total_members: usize,
    /// Members by role
    pub members_by_role: HashMap<String, usize>,
    /// Members by level
    pub members_by_level: HashMap<RoleLevel, usize>,
    /// Average tenure in days
    pub average_tenure_days: u64,
    // TODO: Location count should be handled by composition with cim-domain-location
    // pub location_count: usize,
    /// Child organization count
    pub child_organization_count: usize,
    /// Maximum reporting depth
    pub reporting_depth: usize,
}

/// Organization chart visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationChartView {
    pub organization_id: Uuid,
    pub nodes: Vec<ChartNode>,
    pub edges: Vec<ChartEdge>,
    pub layout_type: String,
}

/// Chart node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartNode {
    pub id: String,
    pub label: String,
    pub node_type: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chart edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartEdge {
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

// TODO: Location distribution should be handled by composition with cim-domain-location
// /// Location distribution view
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LocationDistributionView {
//     pub organization_id: Uuid,
//     pub distributions: Vec<LocationDistribution>,
//     pub total_locations: usize,
// }

// TODO: Location distribution should be handled by composition with cim-domain-location
// /// Location distribution entry
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LocationDistribution {
//     pub location_id: Uuid,
//     pub location_name: String,
//     pub member_count: usize,
//     pub percentage: f32,
// }

/// Size distribution view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeDistributionView {
    pub organization_id: Uuid,
    pub distributions: Vec<SizeDistribution>,
}

/// Size distribution entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeDistribution {
    pub size_category: SizeCategory,
    pub count: usize,
    pub percentage: f32,
}

/// Role distribution view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDistributionView {
    pub organization_id: Uuid,
    pub distributions: Vec<RoleDistribution>,
}

/// Role distribution entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDistribution {
    pub role_title: String,
    pub role_level: RoleLevel,
    pub count: usize,
    pub percentage: f32,
}

/// Vacant position view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VacantPositionView {
    /// Position ID
    pub position_id: Uuid,
    /// Role for this position
    pub role: OrganizationRole,
    /// Department
    pub department: Option<String>,
    /// Reports to
    pub reports_to: Option<Uuid>,
    /// Date position became vacant
    pub vacant_since: chrono::DateTime<chrono::Utc>,
    /// Previous holder
    pub previous_holder: Option<PersonReference>,
}

/// Reference to a person
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonReference {
    /// Person ID
    pub person_id: Uuid,
    /// Person name
    pub name: String,
}

/// Organization summary for lists
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationSummary {
    /// Organization ID
    pub organization_id: Uuid,
    /// Name
    pub name: String,
    /// Type
    pub org_type: OrganizationType,
    /// Status
    pub status: OrganizationStatus,
    /// Member count
    pub member_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_view() {
        let view = OrganizationView {
            organization_id: Uuid::new_v4(),
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            status: OrganizationStatus::Active,
            parent_id: None,
            child_units: vec![],
            member_count: 10,
            size_category: SizeCategory::Small,
        };

        assert_eq!(view.name, "Test Corp");
        assert_eq!(view.member_count, 10);
    }

    #[test]
    fn test_member_view() {
        let member = MemberView {
            person_id: Uuid::new_v4(),
            person_name: "John Doe".to_string(),
            role: OrganizationRole::software_engineer(),
            reports_to_id: Some(Uuid::new_v4()),
            reports_to_name: Some("Jane Smith".to_string()),
            joined_at: chrono::Utc::now(),
            direct_reports_count: 0,
            is_active: true,
        };

        assert_eq!(member.person_name, "John Doe");
        assert!(member.is_active);
    }

    #[test]
    fn test_chart_edge_types() {
        let edge1 = ChartEdge {
            source: "node1".to_string(),
            target: "node2".to_string(),
            edge_type: "reports_to".to_string(),
            metadata: HashMap::new(),
        };

        let edge2 = ChartEdge {
            source: "node1".to_string(),
            target: "node3".to_string(),
            edge_type: "dotted_line".to_string(),
            metadata: HashMap::new(),
        };

        assert_ne!(edge1.edge_type, edge2.edge_type);
    }
} 