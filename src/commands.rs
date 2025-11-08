//! Organization domain commands
//!
//! Commands represent intentions to change organizational state

use chrono::{DateTime, Utc};
use cim_domain::{
    Command, MessageIdentity,
    EntityId,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{
    Department, DepartmentStatus, Facility, FacilityStatus, FacilityType,
    Organization, OrganizationStatus, OrganizationType,
    Role, RoleStatus, RoleType, Team, TeamStatus, TeamType,
};
use crate::aggregate::OrganizationAggregate;

/// Base organization command enum
/// NOTE: This enum only contains pure organization domain commands.
/// Relationship commands (person-to-role, facility-to-location) belong in separate Association domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command_type")]
pub enum OrganizationCommand {
    CreateOrganization(CreateOrganization),
    UpdateOrganization(UpdateOrganization),
    DissolveOrganization(DissolveOrganization),
    MergeOrganizations(MergeOrganizations),
    ChangeOrganizationStatus(ChangeOrganizationStatus),
    CreateDepartment(CreateDepartment),
    UpdateDepartment(UpdateDepartment),
    RestructureDepartment(RestructureDepartment),
    DissolveDepartment(DissolveDepartment),
    CreateTeam(CreateTeam),
    UpdateTeam(UpdateTeam),
    DisbandTeam(DisbandTeam),
    CreateRole(CreateRole),
    UpdateRole(UpdateRole),
    DeprecateRole(DeprecateRole),
    CreateFacility(CreateFacility),
    UpdateFacility(UpdateFacility),
    RemoveFacility(RemoveFacility),
    AddChildOrganization(AddChildOrganization),
    RemoveChildOrganization(RemoveChildOrganization),
}

impl Command for OrganizationCommand {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        match self {
            OrganizationCommand::CreateOrganization(_) => None, // New aggregate
            OrganizationCommand::UpdateOrganization(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::DissolveOrganization(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::MergeOrganizations(cmd) => Some(EntityId::from_uuid(cmd.surviving_organization_id.clone().into())),
            OrganizationCommand::ChangeOrganizationStatus(cmd) => Some(EntityId::from_uuid(cmd.organization_id)),
            OrganizationCommand::CreateDepartment(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::UpdateDepartment(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::RestructureDepartment(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::DissolveDepartment(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::CreateTeam(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::UpdateTeam(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::DisbandTeam(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::CreateRole(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::UpdateRole(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::DeprecateRole(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::CreateFacility(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::UpdateFacility(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::RemoveFacility(cmd) => Some(EntityId::from_uuid(cmd.organization_id.clone().into())),
            OrganizationCommand::AddChildOrganization(cmd) => Some(EntityId::from_uuid(cmd.parent_organization_id)),
            OrganizationCommand::RemoveChildOrganization(cmd) => Some(EntityId::from_uuid(cmd.parent_organization_id)),
        }
    }
}

// Organization commands

/// Command: Create a new organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganization {
    pub identity: MessageIdentity,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub organization_type: OrganizationType,
    pub parent_id: Option<EntityId<Organization>>,
    pub founded_date: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
}

impl Command for CreateOrganization {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        None // Creating new aggregate
    }
}

/// Command: Update organization details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrganization {
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub status: Option<OrganizationStatus>,
    pub metadata: Option<serde_json::Value>,
}

impl Command for UpdateOrganization {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Dissolve organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DissolveOrganization {
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub effective_date: DateTime<Utc>,
}

impl Command for DissolveOrganization {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Merge two organizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeOrganizations {
    pub identity: MessageIdentity,
    pub surviving_organization_id: EntityId<Organization>,
    pub merged_organization_id: EntityId<Organization>,
    pub merger_type: crate::events::MergerType,
    pub effective_date: DateTime<Utc>,
}

impl Command for MergeOrganizations {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.surviving_organization_id.clone().into()))
    }
}

// Department commands

/// Command: Create department
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartment {
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub parent_department_id: Option<EntityId<Department>>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

impl Command for CreateDepartment {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Update department
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartment {
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub head_role_id: Option<EntityId<Role>>,
    pub status: Option<DepartmentStatus>,
}

impl Command for UpdateDepartment {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Restructure department
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestructureDepartment {
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub new_parent_id: Option<EntityId<Department>>,
    pub restructure_type: crate::events::RestructureType,
}

impl Command for RestructureDepartment {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Dissolve department
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DissolveDepartment {
    pub identity: MessageIdentity,
    pub department_id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub transfer_to: Option<EntityId<Department>>,
}

impl Command for DissolveDepartment {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

// Team commands

/// Command: Create team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeam {
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub department_id: Option<EntityId<Department>>,
    pub name: String,
    pub description: Option<String>,
    pub team_type: TeamType,
    pub max_members: Option<usize>,
}

impl Command for CreateTeam {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Update team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTeam {
    pub identity: MessageIdentity,
    pub team_id: EntityId<Team>,
    pub organization_id: EntityId<Organization>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub lead_role_id: Option<EntityId<Role>>,
    pub max_members: Option<usize>,
    pub status: Option<TeamStatus>,
}

impl Command for UpdateTeam {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Disband team
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisbandTeam {
    pub identity: MessageIdentity,
    pub team_id: EntityId<Team>,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub members_transfer_to: Option<EntityId<Team>>,
}

impl Command for DisbandTeam {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

// Role commands

/// Command: Create role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRole {
    pub identity: MessageIdentity,
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
}

impl Command for CreateRole {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Update role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRole {
    pub identity: MessageIdentity,
    pub role_id: EntityId<Role>,
    pub organization_id: EntityId<Organization>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub level: Option<u8>,
    pub reports_to: Option<EntityId<Role>>,
    pub permissions: Option<Vec<String>>,
    pub responsibilities: Option<Vec<String>>,
    pub status: Option<RoleStatus>,
}

impl Command for UpdateRole {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Deprecate role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecateRole {
    pub identity: MessageIdentity,
    pub role_id: EntityId<Role>,
    pub organization_id: EntityId<Organization>,
    pub reason: String,
    pub replacement_role_id: Option<EntityId<Role>>,
    pub effective_date: DateTime<Utc>,
}

impl Command for DeprecateRole {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

// Facility commands - pure organizational places (no location/address data)

/// Command: Create facility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFacility {
    pub identity: MessageIdentity,
    pub organization_id: EntityId<Organization>,
    pub name: String,
    pub code: String,
    pub facility_type: FacilityType,
    pub description: Option<String>,
    pub capacity: Option<u32>,
    pub parent_facility_id: Option<EntityId<Facility>>,
}

impl Command for CreateFacility {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Update facility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFacility {
    pub identity: MessageIdentity,
    pub facility_id: EntityId<Facility>,
    pub organization_id: EntityId<Organization>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<u32>,
    pub status: Option<FacilityStatus>,
    pub parent_facility_id: Option<EntityId<Facility>>,
}

impl Command for UpdateFacility {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

/// Command: Remove facility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveFacility {
    pub identity: MessageIdentity,
    pub facility_id: EntityId<Facility>,
    pub organization_id: EntityId<Organization>,
    pub reason: Option<String>,
}

impl Command for RemoveFacility {
    type Aggregate = OrganizationAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.organization_id.clone().into()))
    }
}

// Hierarchy commands

/// Command: Add child organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddChildOrganization {
    pub identity: MessageIdentity,
    pub parent_organization_id: Uuid,
    pub child_organization_id: Uuid,
    pub child_name: String,
    pub child_type: OrganizationType,
}

/// Command: Remove child organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveChildOrganization {
    pub identity: MessageIdentity,
    pub parent_organization_id: Uuid,
    pub child_organization_id: Uuid,
}

// Status commands

/// Command: Change organization status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeOrganizationStatus {
    pub identity: MessageIdentity,
    pub organization_id: Uuid,
    pub new_status: OrganizationStatus,
    pub reason: Option<String>,
}