//! Organization aggregate root
//!
//! The aggregate root for the organization domain, handling commands
//! and emitting events.

use chrono::Utc;
use cim_domain::{
    AggregateRoot, EntityId,
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

/// Organization aggregate root
///
/// Manages the consistency boundary for organization operations
/// The Organization entity is the aggregate root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAggregate {
    pub organization: Option<Organization>,  // The root entity
    pub departments: HashMap<EntityId<Department>, Department>,
    pub teams: HashMap<EntityId<Team>, Team>,
    pub roles: HashMap<EntityId<Role>, Role>,
    pub version: u64,
}

impl OrganizationAggregate {
    /// Create a new aggregate
    pub fn new() -> Self {
        Self {
            organization: None,
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
            version: 0,
        }
    }

    /// Create aggregate with existing organization
    pub fn from_organization(org: Organization) -> Self {
        Self {
            organization: Some(org),
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
            version: 0,
        }
    }

    /// Get the aggregate root ID (Organization ID if it exists)
    pub fn aggregate_id(&self) -> Option<EntityId<Organization>> {
        self.organization.as_ref().map(|org| org.id.clone())
    }

    /// Handle organization commands
    pub fn handle_command(&mut self, command: OrganizationCommand) -> OrganizationResult<Vec<OrganizationEvent>> {
        match command {
            OrganizationCommand::CreateOrganization(cmd) => self.handle_create_organization(cmd),
            OrganizationCommand::UpdateOrganization(cmd) => self.handle_update_organization(cmd),
            OrganizationCommand::DissolveOrganization(cmd) => self.handle_dissolve_organization(cmd),
            OrganizationCommand::MergeOrganizations(cmd) => self.handle_merge_organizations(cmd),
            OrganizationCommand::CreateDepartment(cmd) => self.handle_create_department(cmd),
            OrganizationCommand::UpdateDepartment(cmd) => self.handle_update_department(cmd),
            OrganizationCommand::RestructureDepartment(cmd) => self.handle_restructure_department(cmd),
            OrganizationCommand::DissolveDepartment(cmd) => self.handle_dissolve_department(cmd),
            OrganizationCommand::CreateTeam(cmd) => self.handle_create_team(cmd),
            OrganizationCommand::UpdateTeam(cmd) => self.handle_update_team(cmd),
            OrganizationCommand::DisbandTeam(cmd) => self.handle_disband_team(cmd),
            OrganizationCommand::CreateRole(cmd) => self.handle_create_role(cmd),
            OrganizationCommand::UpdateRole(cmd) => self.handle_update_role(cmd),
            OrganizationCommand::AssignRole(cmd) => self.handle_assign_role(cmd),
            OrganizationCommand::VacateRole(cmd) => self.handle_vacate_role(cmd),
            OrganizationCommand::DeprecateRole(cmd) => self.handle_deprecate_role(cmd),
        }
    }

    /// Apply an event to update aggregate state
    pub fn apply_event(&mut self, event: &OrganizationEvent) {
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
                self.organization = Some(org);
            }
            OrganizationEvent::OrganizationUpdated(e) => {
                if let Some(org) = &mut self.organization {
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
                self.departments.insert(e.department_id.clone(), dept);
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
                self.teams.insert(e.team_id.clone(), team);
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
                self.roles.insert(e.role_id.clone(), role);
            }
            // Handle other events...
            _ => {}
        }

        self.version += 1;
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

    fn handle_assign_role(&mut self, cmd: AssignRole) -> OrganizationResult<Vec<OrganizationEvent>> {
        let event = RoleAssigned {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            role_id: cmd.role_id,
            organization_id: cmd.organization_id,
            person_id: cmd.person_id,
            effective_date: cmd.effective_date,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::RoleAssigned(event)])
    }

    fn handle_vacate_role(&mut self, cmd: VacateRole) -> OrganizationResult<Vec<OrganizationEvent>> {
        let event = RoleVacated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            role_id: cmd.role_id,
            organization_id: cmd.organization_id,
            vacated_by: cmd.person_id,
            reason: cmd.reason,
            effective_date: cmd.effective_date,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::RoleVacated(event)])
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
}

impl AggregateRoot for OrganizationAggregate {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        // Return the Organization's ID if it exists, otherwise a default (this shouldn't happen in practice)
        self.organization
            .as_ref()
            .map(|org| org.id.clone().into())
            .unwrap_or_else(|| Uuid::nil())
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
        Self::new()
    }
}