//! Organization domain events
//!
//! Events that represent state changes in the organization domain

use chrono::{DateTime, Utc};
use cim_domain::{
    MessageIdentity,
    EntityId,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{
    Department, DepartmentStatus, Facility, FacilityStatus, FacilityType,
    Organization, OrganizationStatus, OrganizationType,
    Role, RoleStatus, RoleType, Team, TeamStatus, TeamType,
};

/// Aggregate of all organization domain events
/// NOTE: This enum only contains pure organization domain events.
/// Relationship events (person-to-role, facility-to-location) belong in separate Association domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum OrganizationEvent {
    OrganizationCreated(OrganizationCreated),
    OrganizationUpdated(OrganizationUpdated),
    OrganizationDissolved(OrganizationDissolved),
    OrganizationMerged(OrganizationMerged),
    OrganizationStatusChanged(OrganizationStatusChanged),
    DepartmentCreated(DepartmentCreated),
    DepartmentUpdated(DepartmentUpdated),
    DepartmentRestructured(DepartmentRestructured),
    DepartmentDissolved(DepartmentDissolved),
    TeamFormed(TeamFormed),
    TeamUpdated(TeamUpdated),
    TeamDisbanded(TeamDisbanded),
    RoleCreated(RoleCreated),
    RoleUpdated(RoleUpdated),
    RoleDeprecated(RoleDeprecated),
    FacilityCreated(FacilityCreated),
    FacilityUpdated(FacilityUpdated),
    FacilityRemoved(FacilityRemoved),
    ChildOrganizationAdded(ChildOrganizationAdded),
    ChildOrganizationRemoved(ChildOrganizationRemoved),
}

impl cim_domain::DomainEvent for OrganizationEvent {
    fn aggregate_id(&self) -> Uuid {
        match self {
            OrganizationEvent::OrganizationCreated(e) => e.organization_id.clone().into(),
            OrganizationEvent::OrganizationUpdated(e) => e.organization_id.clone().into(),
            OrganizationEvent::OrganizationDissolved(e) => e.organization_id.clone().into(),
            OrganizationEvent::OrganizationMerged(e) => e.surviving_organization_id.clone().into(),
            OrganizationEvent::OrganizationStatusChanged(e) => e.organization_id.clone().into(),
            OrganizationEvent::DepartmentCreated(e) => e.organization_id.clone().into(),
            OrganizationEvent::DepartmentUpdated(e) => e.organization_id.clone().into(),
            OrganizationEvent::DepartmentRestructured(e) => e.organization_id.clone().into(),
            OrganizationEvent::DepartmentDissolved(e) => e.organization_id.clone().into(),
            OrganizationEvent::TeamFormed(e) => e.organization_id.clone().into(),
            OrganizationEvent::TeamUpdated(e) => e.organization_id.clone().into(),
            OrganizationEvent::TeamDisbanded(e) => e.organization_id.clone().into(),
            OrganizationEvent::RoleCreated(e) => e.organization_id.clone().into(),
            OrganizationEvent::RoleUpdated(e) => e.organization_id.clone().into(),
            OrganizationEvent::RoleDeprecated(e) => e.organization_id.clone().into(),
            OrganizationEvent::FacilityCreated(e) => e.organization_id.clone().into(),
            OrganizationEvent::FacilityUpdated(e) => e.organization_id.clone().into(),
            OrganizationEvent::FacilityRemoved(e) => e.organization_id.clone().into(),
            OrganizationEvent::ChildOrganizationAdded(e) => e.parent_organization_id.clone().into(),
            OrganizationEvent::ChildOrganizationRemoved(e) => e.parent_organization_id.clone().into(),
        }
    }

    fn event_type(&self) -> &'static str {
        match self {
            OrganizationEvent::OrganizationCreated(_) => "OrganizationCreated",
            OrganizationEvent::OrganizationUpdated(_) => "OrganizationUpdated",
            OrganizationEvent::OrganizationDissolved(_) => "OrganizationDissolved",
            OrganizationEvent::OrganizationMerged(_) => "OrganizationMerged",
            OrganizationEvent::OrganizationStatusChanged(_) => "OrganizationStatusChanged",
            OrganizationEvent::DepartmentCreated(_) => "DepartmentCreated",
            OrganizationEvent::DepartmentUpdated(_) => "DepartmentUpdated",
            OrganizationEvent::DepartmentRestructured(_) => "DepartmentRestructured",
            OrganizationEvent::DepartmentDissolved(_) => "DepartmentDissolved",
            OrganizationEvent::TeamFormed(_) => "TeamFormed",
            OrganizationEvent::TeamUpdated(_) => "TeamUpdated",
            OrganizationEvent::TeamDisbanded(_) => "TeamDisbanded",
            OrganizationEvent::RoleCreated(_) => "RoleCreated",
            OrganizationEvent::RoleUpdated(_) => "RoleUpdated",
            OrganizationEvent::RoleDeprecated(_) => "RoleDeprecated",
            OrganizationEvent::FacilityCreated(_) => "FacilityCreated",
            OrganizationEvent::FacilityUpdated(_) => "FacilityUpdated",
            OrganizationEvent::FacilityRemoved(_) => "FacilityRemoved",
            OrganizationEvent::ChildOrganizationAdded(_) => "ChildOrganizationAdded",
            OrganizationEvent::ChildOrganizationRemoved(_) => "ChildOrganizationRemoved",
        }
    }
}

// Organization lifecycle events

/// Event: Organization created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationCreated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub name: String,
    pub display_name: String,
    pub organization_type: OrganizationType,
    pub parent_id: Option<EntityId<Organization>>,
    pub metadata: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}



/// Event: Organization updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub changes: OrganizationChanges,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationChanges {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub status: Option<OrganizationStatus>,
    pub metadata: Option<serde_json::Value>,
}



/// Event: Organization dissolved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationDissolved {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub effective_date: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}



/// Event: Organizations merged
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMerged {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub surviving_organization_id: EntityId<Organization>,
    pub merged_organization_id: EntityId<Organization>,
    pub merger_type: MergerType,
    pub effective_date: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergerType {
    Acquisition,
    Merger,
    Consolidation,
    Absorption,
}



// Department events

/// Event: Department created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentCreated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub parent_department_id: Option<EntityId<Department>>,
    pub name: String,
    pub code: String,
    pub occurred_at: DateTime<Utc>,
}



/// Event: Department updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub changes: DepartmentChanges,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentChanges {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub head_role_id: Option<EntityId<Role>>,
    pub status: Option<DepartmentStatus>,
}



/// Event: Department restructured
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentRestructured {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub new_parent_id: Option<EntityId<Department>>,
    pub restructure_type: RestructureType,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestructureType {
    Promotion,  // Department promoted in hierarchy
    Demotion,   // Department demoted in hierarchy
    Transfer,   // Department moved to different parent
    Split,      // Department split into multiple
    Merge,      // Department merged with another
}



/// Event: Department dissolved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentDissolved {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub transfer_to: Option<EntityId<Department>>,
    pub occurred_at: DateTime<Utc>,
}



// Team events

/// Event: Team formed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamFormed {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub team_id: EntityId<Team>,
    pub organization_id: EntityId<Organization>,
    pub department_id: Option<EntityId<Department>>,
    pub name: String,
    pub team_type: TeamType,
    pub occurred_at: DateTime<Utc>,
}



/// Event: Team updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub team_id: EntityId<Team>,
    pub organization_id: EntityId<Organization>,
    pub changes: TeamChanges,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamChanges {
    pub name: Option<String>,
    pub description: Option<String>,
    pub lead_role_id: Option<EntityId<Role>>,
    pub max_members: Option<usize>,
    pub status: Option<TeamStatus>,
}



/// Event: Team disbanded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamDisbanded {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub team_id: EntityId<Team>,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub members_transferred_to: Option<EntityId<Team>>,
    pub occurred_at: DateTime<Utc>,
}



// Role events

/// Event: Role created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCreated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub role_id: EntityId<Role>,
    pub organization_id: EntityId<Organization>,
    pub department_id: Option<EntityId<Department>>,
    pub team_id: Option<EntityId<Team>>,
    pub title: String,
    pub code: String,
    pub description: Option<String>,
    pub role_type: RoleType,
    pub level: Option<u8>,
    pub reports_to: Option<EntityId<Role>>,
    pub permissions: Vec<String>,
    pub responsibilities: Vec<String>,
    pub occurred_at: DateTime<Utc>,
}



/// Event: Role updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub role_id: EntityId<Role>,
    pub organization_id: EntityId<Organization>,
    pub changes: RoleChanges,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleChanges {
    pub title: Option<String>,
    pub description: Option<String>,
    pub level: Option<u8>,
    pub reports_to: Option<EntityId<Role>>,
    pub permissions: Option<Vec<String>>,
    pub responsibilities: Option<Vec<String>>,
    pub status: Option<RoleStatus>,
}



/// Event: Role deprecated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDeprecated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub role_id: EntityId<Role>,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub replacement_role_id: Option<EntityId<Role>>,
    pub effective_date: DateTime<Utc>,
    pub occurred_at: DateTime<Utc>,
}



// Facility events - pure organizational places (no location/address data)

/// Event: Facility created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityCreated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub facility_id: EntityId<Facility>,
    pub organization_id: EntityId<Organization>,
    pub name: String,
    pub code: String,
    pub facility_type: FacilityType,
    pub description: Option<String>,
    pub capacity: Option<u32>,
    pub parent_facility_id: Option<EntityId<Facility>>,
    pub occurred_at: DateTime<Utc>,
}



/// Event: Facility updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub facility_id: EntityId<Facility>,
    pub organization_id: EntityId<Organization>,
    pub changes: FacilityChanges,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityChanges {
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<u32>,
    pub status: Option<FacilityStatus>,
    pub parent_facility_id: Option<EntityId<Facility>>,
}



/// Event: Facility removed from organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityRemoved {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub facility_id: EntityId<Facility>,
    pub organization_id: EntityId<Organization>,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Event: Organization status changed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStatusChanged {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub new_status: crate::entity::OrganizationStatus,
    pub previous_status: crate::entity::OrganizationStatus,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Event: Child organization added
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildOrganizationAdded {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub parent_organization_id: EntityId<Organization>,
    pub child_organization_id: Uuid,
    pub child_name: String,
    pub child_type: crate::entity::OrganizationType,
    pub occurred_at: DateTime<Utc>,
}

/// Event: Child organization removed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildOrganizationRemoved {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub parent_organization_id: EntityId<Organization>,
    pub child_organization_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}


