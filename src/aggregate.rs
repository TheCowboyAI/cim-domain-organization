//! Organization aggregate root
//!
//! The aggregate root for the organization domain, handling commands
//! and emitting events.

use chrono::Utc;
use cim_domain::{
    AggregateRoot, EntityId, MealyStateMachine,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    commands::*,
    entity::*,
    events::*,
    OrganizationError, OrganizationResult,
};

/// Organization aggregate state for MealyStateMachine
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationState {
    /// Organization being created
    Creating,
    /// Organization pending activation
    Pending,
    /// Organization active and operational
    Active,
    /// Organization temporarily inactive
    Inactive,
    /// Organization suspended (regulatory or policy violation)
    Suspended,
    /// Organization dissolved (terminal state)
    Dissolved,
    /// Organization merged into another (terminal state)
    Merged,
}

impl From<OrganizationStatus> for OrganizationState {
    fn from(status: OrganizationStatus) -> Self {
        match status {
            OrganizationStatus::Pending => OrganizationState::Pending,
            OrganizationStatus::Active => OrganizationState::Active,
            OrganizationStatus::Inactive => OrganizationState::Inactive,
            OrganizationStatus::Suspended => OrganizationState::Suspended,
            OrganizationStatus::Dissolved => OrganizationState::Dissolved,
            OrganizationStatus::Merged => OrganizationState::Merged,
        }
    }
}

/// Organization aggregate root
///
/// Manages the consistency boundary for organization operations
/// The Organization entity is the aggregate root
///
/// NOTE: This aggregate only contains pure organization domain concepts.
/// Relationships to people and locations are managed in separate domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAggregate {
    pub id: Uuid,
    pub name: String,
    pub org_type: OrganizationType,
    pub status: OrganizationStatus,
    pub child_organizations: HashMap<Uuid, ChildOrganization>,
    pub organization: Option<Organization>,  // The root entity
    pub departments: HashMap<EntityId<Department>, Department>,
    pub teams: HashMap<EntityId<Team>, Team>,
    pub roles: HashMap<EntityId<Role>, Role>,
    pub facilities: HashMap<EntityId<Facility>, Facility>,
    pub version: u64,
}

/// Child organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildOrganization {
    pub id: Uuid,
    pub name: String,
    pub org_type: OrganizationType,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Permissions that can be assigned to roles (organization domain)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Permission {
    CreateOrganization,
    DeleteOrganization,
    ModifyOrganization,
    ViewOrganization,
    ApproveBudget,
    ManageDepartment,
    ManageTeam,
    CreateRole,
    ModifyRole,
    CreateFacility,
    ModifyFacility,
    ViewReports,
    // NOTE: Person assignment permissions are in the Association domain
    // NOT here - Organization domain doesn't manage people
}

impl OrganizationAggregate {
    /// Create an empty aggregate (used when creating organization via command)
    pub fn empty() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: String::new(),
            org_type: OrganizationType::Corporation,
            status: OrganizationStatus::Pending,
            child_organizations: HashMap::new(),
            organization: None,
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
            facilities: HashMap::new(),
            version: 0,
        }
    }

    /// Create a new aggregate with organization details
    pub fn new(id: Uuid, name: String, org_type: OrganizationType) -> Self {
        let org = Organization {
            id: EntityId::from_uuid(id),
            name: name.clone(),
            display_name: name.clone(),
            description: None,
            parent_id: None,
            organization_type: org_type.clone(),
            status: OrganizationStatus::Pending,
            founded_date: None,
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Self {
            id,
            name,
            org_type,
            status: OrganizationStatus::Pending,
            child_organizations: HashMap::new(),
            organization: Some(org),
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
            facilities: HashMap::new(),
            version: 0,
        }
    }

    /// Create aggregate with existing organization
    pub fn from_organization(org: Organization) -> Self {
        Self {
            id: org.id.clone().into(),
            name: org.name.clone(),
            org_type: org.organization_type.clone(),
            status: org.status.clone(),
            child_organizations: HashMap::new(),
            organization: Some(org),
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
            facilities: HashMap::new(),
            version: 0,
        }
    }

    /// Get the aggregate root ID (Organization ID if it exists)
    pub fn aggregate_id(&self) -> Option<EntityId<Organization>> {
        self.organization.as_ref().map(|org| org.id.clone())
    }

    /// Get the current state of the organization for MealyStateMachine
    pub fn current_state(&self) -> OrganizationState {
        if self.organization.is_none() {
            OrganizationState::Creating
        } else {
            OrganizationState::from(self.status.clone())
        }
    }

    /// Handle organization commands
    /// NOTE: This only handles pure organization domain commands.
    /// Relationship commands (person-to-role, facility-to-location) are handled in separate Association domain.
    pub fn handle_command(&mut self, command: OrganizationCommand) -> OrganizationResult<Vec<OrganizationEvent>> {
        match command {
            OrganizationCommand::CreateOrganization(cmd) => self.handle_create_organization(cmd),
            OrganizationCommand::UpdateOrganization(cmd) => self.handle_update_organization(cmd),
            OrganizationCommand::DissolveOrganization(cmd) => self.handle_dissolve_organization(cmd),
            OrganizationCommand::MergeOrganizations(cmd) => self.handle_merge_organizations(cmd),
            OrganizationCommand::ChangeOrganizationStatus(cmd) => self.handle_change_organization_status(cmd),
            OrganizationCommand::CreateDepartment(cmd) => self.handle_create_department(cmd),
            OrganizationCommand::UpdateDepartment(cmd) => self.handle_update_department(cmd),
            OrganizationCommand::RestructureDepartment(cmd) => self.handle_restructure_department(cmd),
            OrganizationCommand::DissolveDepartment(cmd) => self.handle_dissolve_department(cmd),
            OrganizationCommand::CreateTeam(cmd) => self.handle_create_team(cmd),
            OrganizationCommand::UpdateTeam(cmd) => self.handle_update_team(cmd),
            OrganizationCommand::DisbandTeam(cmd) => self.handle_disband_team(cmd),
            OrganizationCommand::CreateRole(cmd) => self.handle_create_role(cmd),
            OrganizationCommand::UpdateRole(cmd) => self.handle_update_role(cmd),
            OrganizationCommand::DeprecateRole(cmd) => self.handle_deprecate_role(cmd),
            OrganizationCommand::CreateFacility(cmd) => self.handle_create_facility(cmd),
            OrganizationCommand::UpdateFacility(cmd) => self.handle_update_facility(cmd),
            OrganizationCommand::RemoveFacility(cmd) => self.handle_remove_facility(cmd),
            OrganizationCommand::AddChildOrganization(cmd) => self.handle_add_child_organization(cmd),
            OrganizationCommand::RemoveChildOrganization(cmd) => self.handle_remove_child_organization(cmd),
        }
    }

    /// Apply an event to create new aggregate state (pure function)
    pub fn apply_event_pure(&self, event: &OrganizationEvent) -> OrganizationResult<Self> {
        let mut new_aggregate = self.clone();
        match event {
            OrganizationEvent::OrganizationCreated(e) => {
                let org = Organization {
                    id: e.organization_id.clone(),
                    name: e.name.clone(),
                    display_name: e.display_name.clone(),
                    description: None,
                    parent_id: e.parent_id.clone(),
                    organization_type: e.organization_type.clone(),
                    status: OrganizationStatus::Active,
                    founded_date: None,
                    metadata: e.metadata.clone(),
                    created_at: e.occurred_at,
                    updated_at: e.occurred_at,
                };
                new_aggregate.organization = Some(org);
                new_aggregate.status = OrganizationStatus::Active;
            }
            OrganizationEvent::OrganizationUpdated(e) => {
                if let Some(org) = &mut new_aggregate.organization {
                    if let Some(name) = &e.changes.name {
                        org.name = name.clone();
                    }
                    if let Some(display_name) = &e.changes.display_name {
                        org.display_name = display_name.clone();
                    }
                    if let Some(description) = &e.changes.description {
                        org.description = Some(description.clone());
                    }
                    if let Some(status) = &e.changes.status {
                        org.status = status.clone();
                    }
                    org.updated_at = e.occurred_at;
                }
            }
            OrganizationEvent::DepartmentCreated(e) => {
                let dept = Department {
                    id: e.department_id.clone(),
                    organization_id: e.organization_id.clone(),
                    parent_department_id: e.parent_department_id.clone(),
                    name: e.name.clone(),
                    code: e.code.clone(),
                    description: None,
                    head_role_id: None,
                    status: DepartmentStatus::Active,
                    created_at: e.occurred_at,
                    updated_at: e.occurred_at,
                };
                new_aggregate.departments.insert(e.department_id.clone(), dept);
            }
            OrganizationEvent::TeamFormed(e) => {
                let team = Team {
                    id: e.team_id.clone(),
                    organization_id: e.organization_id.clone(),
                    department_id: e.department_id.clone(),
                    name: e.name.clone(),
                    description: None,
                    team_type: e.team_type.clone(),
                    lead_role_id: None,
                    max_members: None,
                    status: TeamStatus::Forming,
                    created_at: e.occurred_at,
                    updated_at: e.occurred_at,
                };
                new_aggregate.teams.insert(e.team_id.clone(), team);
            }
            OrganizationEvent::RoleCreated(e) => {
                let role = Role {
                    id: e.role_id.clone(),
                    organization_id: e.organization_id.clone(),
                    department_id: e.department_id.clone(),
                    team_id: e.team_id.clone(),
                    title: e.title.clone(),
                    code: e.code.clone(),
                    description: None,
                    role_type: e.role_type.clone(),
                    level: None,
                    reports_to: None,
                    permissions: Vec::new(),
                    responsibilities: Vec::new(),
                    status: RoleStatus::Active,
                    created_at: e.occurred_at,
                    updated_at: e.occurred_at,
                };
                new_aggregate.roles.insert(e.role_id.clone(), role);
            }
            OrganizationEvent::FacilityCreated(e) => {
                let facility = Facility {
                    id: e.facility_id.clone(),
                    organization_id: e.organization_id.clone(),
                    name: e.name.clone(),
                    code: e.code.clone(),
                    facility_type: e.facility_type.clone(),
                    description: e.description.clone(),
                    capacity: None,
                    status: FacilityStatus::Active,
                    parent_facility_id: None,
                    created_at: e.occurred_at,
                    updated_at: e.occurred_at,
                };
                new_aggregate.facilities.insert(e.facility_id.clone(), facility);
            }
            OrganizationEvent::FacilityUpdated(e) => {
                if let Some(facility) = new_aggregate.facilities.get_mut(&e.facility_id) {
                    if let Some(name) = &e.changes.name {
                        facility.name = name.clone();
                    }
                    if let Some(code) = &e.changes.code {
                        facility.code = code.clone();
                    }
                    if let Some(description) = &e.changes.description {
                        facility.description = Some(description.clone());
                    }
                    if let Some(capacity) = e.changes.capacity {
                        facility.capacity = Some(capacity);
                    }
                    if let Some(status) = &e.changes.status {
                        facility.status = status.clone();
                    }
                    if let Some(parent_facility_id) = &e.changes.parent_facility_id {
                        facility.parent_facility_id = Some(parent_facility_id.clone());
                    }
                    facility.updated_at = e.occurred_at;
                }
            }
            OrganizationEvent::FacilityRemoved(e) => {
                new_aggregate.facilities.remove(&e.facility_id);
            }
            OrganizationEvent::OrganizationStatusChanged(e) => {
                new_aggregate.status = e.new_status.clone();
                if let Some(org) = &mut new_aggregate.organization {
                    org.status = e.new_status.clone();
                }
            }
            OrganizationEvent::OrganizationDissolved(_e) => {
                new_aggregate.status = OrganizationStatus::Dissolved;
                if let Some(org) = &mut new_aggregate.organization {
                    org.status = OrganizationStatus::Dissolved;
                }
            }
            OrganizationEvent::OrganizationMerged(_e) => {
                new_aggregate.status = OrganizationStatus::Merged;
                if let Some(org) = &mut new_aggregate.organization {
                    org.status = OrganizationStatus::Merged;
                }
            }
            OrganizationEvent::ChildOrganizationAdded(e) => {
                let child = ChildOrganization {
                    id: e.child_organization_id,
                    name: e.child_name.clone(),
                    org_type: e.child_type.clone(),
                    added_at: e.occurred_at,
                };
                new_aggregate.child_organizations.insert(e.child_organization_id, child);
            }
            OrganizationEvent::ChildOrganizationRemoved(e) => {
                new_aggregate.child_organizations.remove(&e.child_organization_id);
            }
            // Handle other events...
            _ => {}
        }

        new_aggregate.version += 1;
        Ok(new_aggregate)
    }

    /// Apply an event to update aggregate state (mutable wrapper for compatibility)
    /// This is a compatibility wrapper - prefer `apply_event_pure` for pure functional approach
    pub fn apply_event(&mut self, event: &OrganizationEvent) -> OrganizationResult<()> {
        let new_aggregate = self.apply_event_pure(event)?;
        *self = new_aggregate;
        Ok(())
    }

    // Command handlers

    fn handle_create_organization(&mut self, cmd: CreateOrganization) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_some() {
            return Err(OrganizationError::DuplicateEntity("Organization already exists".to_string()));
        }

        let org_id = EntityId::new();
        let event = OrganizationCreated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: org_id,
            name: cmd.name,
            display_name: cmd.display_name,
            organization_type: cmd.organization_type,
            parent_id: cmd.parent_id,
            metadata: cmd.metadata,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::OrganizationCreated(event)])
    }

    fn handle_update_organization(&mut self, cmd: UpdateOrganization) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_none() {
            return Err(OrganizationError::OrganizationNotFound(cmd.organization_id.into()));
        }

        let event = OrganizationUpdated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: cmd.organization_id,
            changes: OrganizationChanges {
                name: cmd.name,
                display_name: cmd.display_name,
                description: cmd.description,
                status: cmd.status,
                metadata: cmd.metadata,
            },
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::OrganizationUpdated(event)])
    }

    fn handle_dissolve_organization(&mut self, cmd: DissolveOrganization) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_none() {
            return Err(OrganizationError::OrganizationNotFound(cmd.organization_id.into()));
        }

        let event = OrganizationDissolved {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: cmd.organization_id,
            reason: cmd.reason,
            effective_date: cmd.effective_date,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::OrganizationDissolved(event)])
    }

    fn handle_merge_organizations(&mut self, cmd: MergeOrganizations) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_none() {
            return Err(OrganizationError::OrganizationNotFound(cmd.surviving_organization_id.into()));
        }

        // Check for self-merge
        if cmd.surviving_organization_id == cmd.merged_organization_id {
            return Err(OrganizationError::CircularReference("Organization cannot merge with itself".to_string()));
        }

        let event = OrganizationMerged {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            surviving_organization_id: cmd.surviving_organization_id,
            merged_organization_id: cmd.merged_organization_id,
            merger_type: cmd.merger_type,
            effective_date: cmd.effective_date,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::OrganizationMerged(event)])
    }

    fn handle_create_department(&mut self, cmd: CreateDepartment) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_none() {
            return Err(OrganizationError::OrganizationNotFound(cmd.organization_id.into()));
        }

        let dept_id = EntityId::new();
        let event = DepartmentCreated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            department_id: dept_id,
            organization_id: cmd.organization_id,
            parent_department_id: cmd.parent_department_id,
            name: cmd.name,
            code: cmd.code,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::DepartmentCreated(event)])
    }

    fn handle_update_department(&mut self, cmd: UpdateDepartment) -> OrganizationResult<Vec<OrganizationEvent>> {
        if !self.departments.contains_key(&cmd.department_id) {
            return Err(OrganizationError::DepartmentNotFound(cmd.department_id.into()));
        }

        let event = DepartmentUpdated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            department_id: cmd.department_id,
            organization_id: cmd.organization_id,
            changes: DepartmentChanges {
                name: cmd.name,
                code: cmd.code,
                description: cmd.description,
                head_role_id: cmd.head_role_id,
                status: cmd.status,
            },
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::DepartmentUpdated(event)])
    }

    fn handle_restructure_department(&mut self, cmd: RestructureDepartment) -> OrganizationResult<Vec<OrganizationEvent>> {
        if !self.departments.contains_key(&cmd.department_id) {
            return Err(OrganizationError::DepartmentNotFound(cmd.department_id.into()));
        }

        let event = DepartmentRestructured {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            department_id: cmd.department_id,
            organization_id: cmd.organization_id,
            new_parent_id: cmd.new_parent_id,
            restructure_type: cmd.restructure_type,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::DepartmentRestructured(event)])
    }

    fn handle_dissolve_department(&mut self, cmd: DissolveDepartment) -> OrganizationResult<Vec<OrganizationEvent>> {
        if !self.departments.contains_key(&cmd.department_id) {
            return Err(OrganizationError::DepartmentNotFound(cmd.department_id.into()));
        }

        let event = DepartmentDissolved {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            department_id: cmd.department_id,
            organization_id: cmd.organization_id,
            reason: cmd.reason,
            transfer_to: cmd.transfer_to,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::DepartmentDissolved(event)])
    }

    fn handle_create_team(&mut self, cmd: CreateTeam) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_none() {
            return Err(OrganizationError::OrganizationNotFound(cmd.organization_id.into()));
        }

        let team_id = EntityId::new();
        let event = TeamFormed {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            team_id,
            organization_id: cmd.organization_id,
            department_id: cmd.department_id,
            name: cmd.name,
            team_type: cmd.team_type,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::TeamFormed(event)])
    }

    fn handle_update_team(&mut self, cmd: UpdateTeam) -> OrganizationResult<Vec<OrganizationEvent>> {
        if !self.teams.contains_key(&cmd.team_id) {
            return Err(OrganizationError::TeamNotFound(cmd.team_id.into()));
        }

        let event = TeamUpdated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            team_id: cmd.team_id,
            organization_id: cmd.organization_id,
            changes: TeamChanges {
                name: cmd.name,
                description: cmd.description,
                lead_role_id: cmd.lead_role_id,
                max_members: cmd.max_members,
                status: cmd.status,
            },
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::TeamUpdated(event)])
    }

    fn handle_disband_team(&mut self, cmd: DisbandTeam) -> OrganizationResult<Vec<OrganizationEvent>> {
        if !self.teams.contains_key(&cmd.team_id) {
            return Err(OrganizationError::TeamNotFound(cmd.team_id.into()));
        }

        let event = TeamDisbanded {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            team_id: cmd.team_id,
            organization_id: cmd.organization_id,
            reason: cmd.reason,
            members_transferred_to: cmd.members_transfer_to,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::TeamDisbanded(event)])
    }

    fn handle_create_role(&mut self, cmd: CreateRole) -> OrganizationResult<Vec<OrganizationEvent>> {
        if self.organization.is_none() {
            return Err(OrganizationError::OrganizationNotFound(cmd.organization_id.into()));
        }

        let role_id = EntityId::new();
        let event = RoleCreated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            role_id,
            organization_id: cmd.organization_id,
            department_id: cmd.department_id,
            team_id: cmd.team_id,
            title: cmd.title,
            code: cmd.code,
            role_type: cmd.role_type,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::RoleCreated(event)])
    }

    fn handle_update_role(&mut self, cmd: UpdateRole) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Validation would go here
        let event = RoleUpdated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            role_id: cmd.role_id,
            organization_id: cmd.organization_id,
            changes: RoleChanges {
                title: cmd.title,
                description: cmd.description,
                level: cmd.level,
                reports_to: cmd.reports_to,
                permissions: cmd.permissions,
                responsibilities: cmd.responsibilities,
                status: cmd.status,
            },
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::RoleUpdated(event)])
    }

    fn handle_deprecate_role(&mut self, cmd: DeprecateRole) -> OrganizationResult<Vec<OrganizationEvent>> {
        let event = RoleDeprecated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            role_id: cmd.role_id,
            organization_id: cmd.organization_id,
            reason: cmd.reason,
            replacement_role_id: cmd.replacement_role_id,
            effective_date: cmd.effective_date,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::RoleDeprecated(event)])
    }

    // Facility management handlers - pure organizational places (no location/address data)

    fn handle_create_facility(&mut self, cmd: CreateFacility) -> OrganizationResult<Vec<OrganizationEvent>> {
        let event = FacilityCreated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            facility_id: EntityId::new(),
            organization_id: cmd.organization_id,
            name: cmd.name,
            code: cmd.code,
            facility_type: cmd.facility_type,
            description: cmd.description,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::FacilityCreated(event)])
    }

    fn handle_update_facility(&mut self, cmd: UpdateFacility) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if facility exists
        if !self.facilities.contains_key(&cmd.facility_id) {
            return Err(OrganizationError::EntityNotFound(format!("Facility {} not found", cmd.facility_id)));
        }

        let event = FacilityUpdated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            facility_id: cmd.facility_id,
            organization_id: cmd.organization_id,
            changes: FacilityChanges {
                name: cmd.name,
                code: cmd.code,
                description: cmd.description,
                capacity: cmd.capacity,
                status: cmd.status,
                parent_facility_id: cmd.parent_facility_id,
            },
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::FacilityUpdated(event)])
    }

    fn handle_remove_facility(&mut self, cmd: RemoveFacility) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if facility exists
        if !self.facilities.contains_key(&cmd.facility_id) {
            return Err(OrganizationError::EntityNotFound(format!("Facility {} not found", cmd.facility_id)));
        }

        let event = FacilityRemoved {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            facility_id: cmd.facility_id,
            organization_id: cmd.organization_id,
            reason: cmd.reason,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::FacilityRemoved(event)])
    }

    // Hierarchy handlers

    fn handle_add_child_organization(&mut self, cmd: AddChildOrganization) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check for self-reference (circular reference)
        if cmd.child_organization_id == self.id {
            return Err(OrganizationError::CircularReference("Organization cannot be its own child".to_string()));
        }

        // Check if child organization already exists
        if self.child_organizations.contains_key(&cmd.child_organization_id) {
            return Err(OrganizationError::DuplicateEntity(cmd.child_organization_id.to_string()));
        }

        let event = crate::events::ChildOrganizationAdded {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            parent_organization_id: EntityId::from_uuid(self.id),
            child_organization_id: cmd.child_organization_id,
            child_name: cmd.child_name,
            child_type: cmd.child_type,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::ChildOrganizationAdded(event)])
    }

    fn handle_remove_child_organization(&mut self, cmd: RemoveChildOrganization) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if child organization exists
        if !self.child_organizations.contains_key(&cmd.child_organization_id) {
            return Err(OrganizationError::OrganizationNotFound(cmd.child_organization_id));
        }

        let event = crate::events::ChildOrganizationRemoved {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            parent_organization_id: EntityId::from_uuid(self.id),
            child_organization_id: cmd.child_organization_id,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::ChildOrganizationRemoved(event)])
    }

    // Status handlers

    fn handle_change_organization_status(&mut self, cmd: ChangeOrganizationStatus) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Validate status transition
        if !self.is_valid_status_transition(self.status.clone(), cmd.new_status.clone()) {
            return Err(OrganizationError::InvalidStructure(
                format!("Invalid status transition from {:?} to {:?}", self.status, cmd.new_status)
            ));
        }

        // Create event
        let event = crate::events::OrganizationStatusChanged {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            new_status: cmd.new_status.clone(),
            previous_status: self.status.clone(),
            reason: cmd.reason,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::OrganizationStatusChanged(event)])
    }

    /// Check if a status transition is valid
    fn is_valid_status_transition(&self, from: OrganizationStatus, to: OrganizationStatus) -> bool {
        use OrganizationStatus::*;

        match (from, to) {
            // Can't transition to the same status
            (a, b) if a == b => false,
            // Pending can transition to Active
            (Pending, Active) => true,
            // Active can transition to Inactive, Suspended, Dissolved, or Merged
            (Active, Inactive) | (Active, Suspended) | (Active, Dissolved) | (Active, Merged) => true,
            // Inactive can transition back to Active (reactivation)
            (Inactive, Active) => true,
            // Suspended can transition to Active (unsuspend) or Dissolved
            (Suspended, Active) | (Suspended, Dissolved) => true,
            // Dissolved and Merged are terminal states - no transitions allowed
            (Dissolved, _) | (Merged, _) => false,
            // All other transitions are invalid
            _ => false,
        }
    }
}

/// MealyStateMachine implementation for OrganizationAggregate
///
/// This implements the pure functional state machine pattern from Category Theory
/// where (State, Input) → (Output, NewState)
impl MealyStateMachine for OrganizationAggregate {
    type State = OrganizationState;
    type Input = OrganizationCommand;
    type Output = Vec<OrganizationEvent>;

    /// Output function: Given current state and input command, produce events
    /// This is a pure function - no side effects, deterministic output
    fn output(&self, _state: Self::State, input: Self::Input) -> Self::Output {
        // Create a clone for pure command handling
        let mut clone = self.clone();

        // Use the existing command handlers on the clone
        match clone.handle_command(input) {
            Ok(events) => events,
            Err(_) => vec![], // Failed commands produce no events
        }
    }

    /// Transition function: Given current state and input command, determine new state
    /// This is a pure function - deterministic state transition
    fn transition(&self, state: Self::State, input: Self::Input) -> Self::State {
        use OrganizationCommand::*;
        use OrganizationState::*;

        match (state, input) {
            // Creating → Pending (organization created)
            (Creating, CreateOrganization(_)) => Pending,

            // Pending → Active (organization activated)
            (Pending, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Active) => Active,

            // Active → Inactive (deactivation)
            (Active, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Inactive) => Inactive,

            // Active → Suspended (suspension)
            (Active, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Suspended) => Suspended,

            // Active → Dissolved (dissolution)
            (Active, DissolveOrganization(_)) => Dissolved,
            (Active, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Dissolved) => Dissolved,

            // Active → Merged (merger)
            (Active, MergeOrganizations(_)) => Merged,
            (Active, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Merged) => Merged,

            // Inactive → Active (reactivation)
            (Inactive, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Active) => Active,

            // Suspended → Active (unsuspend)
            (Suspended, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Active) => Active,

            // Suspended → Dissolved
            (Suspended, DissolveOrganization(_)) => Dissolved,
            (Suspended, ChangeOrganizationStatus(cmd)) if matches!(cmd.new_status, OrganizationStatus::Dissolved) => Dissolved,

            // Terminal states - no transitions
            (Dissolved, _) => Dissolved,
            (Merged, _) => Merged,

            // All other commands don't change state
            (current_state, _) => current_state,
        }
    }
}

impl AggregateRoot for OrganizationAggregate {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }
}

impl Default for OrganizationAggregate {
    fn default() -> Self {
        Self::new(
            Uuid::now_v7(),
            "Default Organization".to_string(),
            OrganizationType::Corporation,
        )
    }
}