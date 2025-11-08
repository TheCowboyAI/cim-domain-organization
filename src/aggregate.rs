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
    pub id: Uuid,
    pub name: String,
    pub org_type: OrganizationType,
    pub status: OrganizationStatus,
    pub members: HashMap<Uuid, OrganizationMember>,
    pub locations: HashMap<Uuid, OrganizationLocation>,
    pub child_organizations: HashMap<Uuid, ChildOrganization>,
    pub organization: Option<Organization>,  // The root entity
    pub departments: HashMap<EntityId<Department>, Department>,
    pub teams: HashMap<EntityId<Team>, Team>,
    pub roles: HashMap<EntityId<Role>, Role>,
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

/// Organization member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationMember {
    pub id: Uuid,
    pub person_id: Uuid,
    pub role: OrganizationRole,
    pub department_id: Option<Uuid>,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

/// Organization role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationRole {
    pub title: String,
    pub level: RoleLevel,
    pub reports_to: Option<Uuid>,
}

impl OrganizationRole {
    pub fn ceo() -> Self {
        Self {
            title: "CEO".to_string(),
            level: RoleLevel::Executive,
            reports_to: None,
        }
    }

    pub fn software_engineer() -> Self {
        Self {
            title: "Software Engineer".to_string(),
            level: RoleLevel::Mid,
            reports_to: None,
        }
    }

    pub fn manager() -> Self {
        Self {
            title: "Manager".to_string(),
            level: RoleLevel::Senior,
            reports_to: None,
        }
    }

    pub fn director() -> Self {
        Self {
            title: "Director".to_string(),
            level: RoleLevel::Executive,
            reports_to: None,
        }
    }

    pub fn engineering_manager() -> Self {
        Self {
            title: "Engineering Manager".to_string(),
            level: RoleLevel::Senior,
            reports_to: None,
        }
    }

    /// Check if this role has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        match self.level {
            RoleLevel::Executive => true, // Executives have all permissions
            RoleLevel::Senior => match permission {
                Permission::CreateOrganization => false,
                Permission::DeleteOrganization => false,
                Permission::ViewOrganization => true,
                Permission::ApproveBudget => true,
                Permission::HireEmployees => true,
                Permission::ManageTeam => true,
                Permission::DevelopSoftware => true,
                Permission::ViewReports => true,
                Permission::AddMember => true,
                Permission::UpdateMemberRole => true,
                Permission::RemoveMember => false,
            },
            RoleLevel::Mid => match permission {
                Permission::CreateOrganization => false,
                Permission::DeleteOrganization => false,
                Permission::ViewOrganization => true,
                Permission::ApproveBudget => false,
                Permission::HireEmployees => false,
                Permission::ManageTeam => true,
                Permission::DevelopSoftware => true,
                Permission::ViewReports => true,
                Permission::AddMember => false,
                Permission::UpdateMemberRole => false,
                Permission::RemoveMember => false,
            },
            RoleLevel::Junior => match permission {
                Permission::CreateOrganization => false,
                Permission::DeleteOrganization => false,
                Permission::ViewOrganization => true,
                Permission::ApproveBudget => false,
                Permission::HireEmployees => false,
                Permission::ManageTeam => false,
                Permission::DevelopSoftware => true,
                Permission::ViewReports => true,
                Permission::AddMember => false,
                Permission::UpdateMemberRole => false,
                Permission::RemoveMember => false,
            },
            RoleLevel::Intern => match permission {
                Permission::CreateOrganization => false,
                Permission::DeleteOrganization => false,
                Permission::ViewOrganization => true,
                Permission::ApproveBudget => false,
                Permission::HireEmployees => false,
                Permission::ManageTeam => false,
                Permission::DevelopSoftware => true,
                Permission::ViewReports => false,
                Permission::AddMember => false,
                Permission::UpdateMemberRole => false,
                Permission::RemoveMember => false,
            },
        }
    }
}

/// Role level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoleLevel {
    Executive,
    Senior,
    Mid,
    Junior,
    Intern,
}

/// Permissions that can be assigned to roles
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    CreateOrganization,
    DeleteOrganization,
    ViewOrganization,
    ApproveBudget,
    HireEmployees,
    ManageTeam,
    DevelopSoftware,
    ViewReports,
    AddMember,
    UpdateMemberRole,
    RemoveMember,
}

/// Organization location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationLocation {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub is_primary: bool,
}

impl OrganizationAggregate {
    /// Create an empty aggregate (used when creating organization via command)
    pub fn empty() -> Self {
        Self {
            id: Uuid::now_v7(),
            name: String::new(),
            org_type: OrganizationType::Corporation,
            status: OrganizationStatus::Pending,
            members: HashMap::new(),
            locations: HashMap::new(),
            child_organizations: HashMap::new(),
            organization: None,
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
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
            members: HashMap::new(),
            locations: HashMap::new(),
            child_organizations: HashMap::new(),
            organization: Some(org),
            departments: HashMap::new(),
            teams: HashMap::new(),
            roles: HashMap::new(),
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
            members: HashMap::new(),
            locations: HashMap::new(),
            child_organizations: HashMap::new(),
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
            // Member management
            OrganizationCommand::AddMember(cmd) => self.handle_add_member(cmd),
            OrganizationCommand::UpdateMemberRole(cmd) => self.handle_update_member_role(cmd),
            OrganizationCommand::RemoveMember(cmd) => self.handle_remove_member(cmd),
            OrganizationCommand::ChangeReportingRelationship(cmd) => self.handle_change_reporting_relationship(cmd),
            // Location management
            OrganizationCommand::AddLocation(cmd) => self.handle_add_location(cmd),
            OrganizationCommand::ChangePrimaryLocation(cmd) => self.handle_change_primary_location(cmd),
            OrganizationCommand::RemoveLocation(cmd) => self.handle_remove_location(cmd),
            // Hierarchy
            OrganizationCommand::AddChildOrganization(cmd) => self.handle_add_child_organization(cmd),
            OrganizationCommand::RemoveChildOrganization(cmd) => self.handle_remove_child_organization(cmd),
            // Status
            OrganizationCommand::ChangeOrganizationStatus(cmd) => self.handle_change_organization_status(cmd),
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
            OrganizationEvent::MemberAdded(e) => {
                let member = OrganizationMember {
                    id: e.person_id,
                    person_id: e.person_id,
                    role: e.role.clone(),
                    department_id: e.department_id,
                    joined_at: e.occurred_at,
                };
                new_aggregate.members.insert(e.person_id, member);
            }
            OrganizationEvent::LocationAdded(e) => {
                let location = OrganizationLocation {
                    id: e.location_id,
                    name: e.name.clone(),
                    address: e.address.clone(),
                    is_primary: e.is_primary,
                };
                new_aggregate.locations.insert(e.location_id, location);
            }
            OrganizationEvent::OrganizationStatusChanged(e) => {
                new_aggregate.status = e.new_status.clone();
                if let Some(org) = &mut new_aggregate.organization {
                    org.status = e.new_status.clone();
                }
            }
            OrganizationEvent::MemberRoleUpdated(e) => {
                if let Some(member) = new_aggregate.members.get_mut(&e.person_id) {
                    member.role = e.new_role.clone();
                }
            }
            OrganizationEvent::MemberRemoved(e) => {
                new_aggregate.members.remove(&e.person_id);
            }
            OrganizationEvent::ReportingRelationshipChanged(e) => {
                if let Some(subordinate) = new_aggregate.members.get_mut(&e.person_id) {
                    subordinate.role.reports_to = e.new_manager_id;
                }
            }
            OrganizationEvent::PrimaryLocationChanged(e) => {
                // Update all locations to set the new primary
                for (id, location) in new_aggregate.locations.iter_mut() {
                    location.is_primary = *id == e.new_primary_location_id;
                }
            }
            OrganizationEvent::LocationRemoved(e) => {
                new_aggregate.locations.remove(&e.location_id);
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

    // Member management handlers

    fn handle_add_member(&mut self, cmd: AddMember) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if member already exists
        if self.members.contains_key(&cmd.person_id) {
            return Err(OrganizationError::DuplicateEntity(format!("Member {} already exists", cmd.person_id)));
        }

        // Create the MemberAdded event
        let event = crate::events::MemberAdded {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            person_id: cmd.person_id,
            role: cmd.role,
            department_id: cmd.department_id,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::MemberAdded(event)])
    }

    fn handle_update_member_role(&mut self, cmd: UpdateMemberRole) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if member exists
        let member = self.members.get(&cmd.person_id)
            .ok_or_else(|| OrganizationError::OrganizationNotFound(cmd.person_id))?;

        // Create event with previous role for history tracking
        let event = crate::events::MemberRoleUpdated {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            person_id: cmd.person_id,
            new_role: cmd.new_role.clone(),
            previous_role: member.role.clone(),
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::MemberRoleUpdated(event)])
    }

    fn handle_remove_member(&mut self, cmd: RemoveMember) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if member exists
        if !self.members.contains_key(&cmd.person_id) {
            return Err(OrganizationError::OrganizationNotFound(cmd.person_id));
        }

        // Create MemberRemoved event
        let event = crate::events::MemberRemoved {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            person_id: cmd.person_id,
            reason: cmd.reason,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::MemberRemoved(event)])
    }

    fn handle_change_reporting_relationship(&mut self, cmd: ChangeReportingRelationship) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check both members exist
        if !self.members.contains_key(&cmd.subordinate_id) {
            return Err(OrganizationError::OrganizationNotFound(cmd.subordinate_id));
        }

        if !self.members.contains_key(&cmd.new_manager_id) {
            return Err(OrganizationError::OrganizationNotFound(cmd.new_manager_id));
        }

        // Check for circular reference - the new manager cannot be a direct or indirect subordinate of the subordinate
        if self.would_create_circular_reference(cmd.subordinate_id, cmd.new_manager_id) {
            return Err(OrganizationError::CircularReference("Cannot create circular reporting relationship".to_string()));
        }

        // Get current manager for history
        let previous_manager = self.members.get(&cmd.subordinate_id)
            .and_then(|m| m.role.reports_to);

        // Create event
        let event = crate::events::ReportingRelationshipChanged {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            person_id: cmd.subordinate_id,
            new_manager_id: Some(cmd.new_manager_id),
            previous_manager_id: previous_manager,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::ReportingRelationshipChanged(event)])
    }

    // Location management handlers

    fn handle_add_location(&mut self, cmd: AddLocation) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if location already exists
        if self.locations.contains_key(&cmd.location_id) {
            return Err(OrganizationError::DuplicateEntity(format!("Location {} already exists", cmd.location_id)));
        }

        // Create LocationAdded event
        let event = crate::events::LocationAdded {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            location_id: cmd.location_id,
            name: cmd.name,
            address: cmd.address,
            is_primary: self.locations.is_empty(), // First location is primary
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::LocationAdded(event)])
    }

    fn handle_change_primary_location(&mut self, cmd: ChangePrimaryLocation) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if location exists
        if !self.locations.contains_key(&cmd.location_id) {
            return Err(OrganizationError::OrganizationNotFound(cmd.location_id));
        }

        // Find the previous primary location
        let previous_primary = self.locations.iter()
            .find(|(_, loc)| loc.is_primary)
            .map(|(id, _)| *id);

        // Create event
        let event = crate::events::PrimaryLocationChanged {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            new_primary_location_id: cmd.location_id,
            previous_primary_location_id: previous_primary,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::PrimaryLocationChanged(event)])
    }

    fn handle_remove_location(&mut self, cmd: RemoveLocation) -> OrganizationResult<Vec<OrganizationEvent>> {
        // Check if location exists
        if !self.locations.contains_key(&cmd.location_id) {
            return Err(OrganizationError::OrganizationNotFound(cmd.location_id));
        }

        // Create event
        let event = crate::events::LocationRemoved {
            event_id: Uuid::now_v7(),
            identity: cmd.identity,
            organization_id: EntityId::from_uuid(cmd.organization_id),
            location_id: cmd.location_id,
            occurred_at: Utc::now(),
        };

        Ok(vec![OrganizationEvent::LocationRemoved(event)])
    }

    // Helper methods

    /// Check if making subordinate report to new_manager would create a circular reference
    fn would_create_circular_reference(&self, subordinate_id: Uuid, new_manager_id: Uuid) -> bool {
        // Start from the new manager and traverse up the reporting chain
        // If we encounter the subordinate, it would create a cycle
        let mut current_id = new_manager_id;
        let mut visited = std::collections::HashSet::new();

        while let Some(member) = self.members.get(&current_id) {
            // Avoid infinite loops in case there's already a cycle
            if !visited.insert(current_id) {
                break;
            }

            // If the new manager reports to the subordinate (directly or indirectly), it's a cycle
            if current_id == subordinate_id {
                return true;
            }

            // Move up the reporting chain
            if let Some(reports_to) = member.role.reports_to {
                current_id = reports_to;
            } else {
                break;
            }
        }

        false
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