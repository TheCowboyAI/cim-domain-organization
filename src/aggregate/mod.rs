//! Organization aggregate root

use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::commands::*;
use crate::events::*;
use crate::value_objects::*;

/// Type alias for organization ID
pub type OrganizationId = Uuid;

/// Organization aggregate root
#[derive(Debug, Clone)]
pub struct OrganizationAggregate {
    /// Unique identifier
    pub id: Uuid,
    /// Organization name
    pub name: String,
    /// Organization type
    pub org_type: OrganizationType,
    /// Current status
    pub status: OrganizationStatus,
    /// Parent organization ID
    pub parent_id: Option<Uuid>,
    /// Child organization IDs
    pub child_units: HashSet<Uuid>,
    /// Organization members
    pub members: HashMap<Uuid, OrganizationMember>,
    /// Associated locations
    pub locations: HashSet<Uuid>,
    /// Primary location
    pub primary_location_id: Option<Uuid>,
    /// Version for optimistic concurrency
    pub version: u64,
}

impl OrganizationAggregate {
    /// Create a new organization aggregate
    pub fn new(id: Uuid, name: String, org_type: OrganizationType) -> Self {
        Self {
            id,
            name,
            org_type,
            status: OrganizationStatus::Pending,
            parent_id: None,
            child_units: HashSet::new(),
            members: HashMap::new(),
            locations: HashSet::new(),
            primary_location_id: None,
            version: 0,
        }
    }

    /// Handle a command and produce events
    pub fn handle_command(&mut self, command: OrganizationCommand) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        match command {
            OrganizationCommand::Create(cmd) => self.handle_create(cmd),
            OrganizationCommand::Update(cmd) => self.handle_update(cmd),
            OrganizationCommand::ChangeStatus(cmd) => self.handle_change_status(cmd),
            OrganizationCommand::AddMember(cmd) => self.handle_add_member(cmd),
            OrganizationCommand::RemoveMember(cmd) => self.handle_remove_member(cmd),
            OrganizationCommand::UpdateMemberRole(cmd) => self.handle_update_member_role(cmd),
            OrganizationCommand::ChangeReportingRelationship(cmd) => self.handle_change_reporting(cmd),
            OrganizationCommand::AddChildOrganization(cmd) => self.handle_add_child(cmd),
            OrganizationCommand::RemoveChildOrganization(cmd) => self.handle_remove_child(cmd),
            OrganizationCommand::AddLocation(cmd) => self.handle_add_location(cmd),
            OrganizationCommand::RemoveLocation(cmd) => self.handle_remove_location(cmd),
            OrganizationCommand::ChangePrimaryLocation(cmd) => self.handle_change_primary_location(cmd),
            OrganizationCommand::Dissolve(cmd) => self.handle_dissolve(cmd),
            OrganizationCommand::Merge(cmd) => self.handle_merge(cmd),
            OrganizationCommand::Acquire(cmd) => self.handle_acquire(cmd),
        }
    }

    /// Apply an event to update state
    pub fn apply_event(&mut self, event: &OrganizationEvent) -> Result<(), OrganizationError> {
        match event {
            OrganizationEvent::Created(e) => self.apply_created(e),
            OrganizationEvent::Updated(e) => self.apply_updated(e),
            OrganizationEvent::StatusChanged(e) => self.apply_status_changed(e),
            OrganizationEvent::MemberAdded(e) => self.apply_member_added(e),
            OrganizationEvent::MemberRemoved(e) => self.apply_member_removed(e),
            OrganizationEvent::MemberRoleUpdated(e) => self.apply_member_role_updated(e),
            OrganizationEvent::ReportingRelationshipChanged(e) => self.apply_reporting_changed(e),
            OrganizationEvent::ChildOrganizationAdded(e) => self.apply_child_added(e),
            OrganizationEvent::ChildOrganizationRemoved(e) => self.apply_child_removed(e),
            OrganizationEvent::LocationAdded(e) => self.apply_location_added(e),
            OrganizationEvent::LocationRemoved(e) => self.apply_location_removed(e),
            OrganizationEvent::PrimaryLocationChanged(e) => self.apply_primary_location_changed(e),
            OrganizationEvent::Dissolved(e) => self.apply_dissolved(e),
            OrganizationEvent::Merged(e) => self.apply_merged(e),
            OrganizationEvent::Acquired(e) => self.apply_acquired(e),
        }
        self.version += 1;
        Ok(())
    }

    // Command handlers

    fn handle_create(&mut self, cmd: CreateOrganization) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Validate command
        if cmd.name.trim().is_empty() {
            return Err(OrganizationError::InvalidName("Organization name cannot be empty".to_string()));
        }

        let event = OrganizationCreated {
            organization_id: cmd.organization_id,
            name: cmd.name,
            org_type: cmd.org_type,
            parent_id: cmd.parent_id,
            primary_location_id: cmd.primary_location_id,
            created_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::Created(event)])
    }

    fn handle_update(&mut self, cmd: UpdateOrganization) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if organization can be modified
        if !self.status.can_be_modified() {
            return Err(OrganizationError::InvalidStatus(
                format!("Cannot update organization in {} status", self.status)
            ));
        }

        // Validate new name if provided
        if let Some(ref name) = cmd.name {
            if name.trim().is_empty() {
                return Err(OrganizationError::InvalidName("Organization name cannot be empty".to_string()));
            }
        }

        let event = OrganizationUpdated {
            organization_id: self.id,
            name: cmd.name,
            primary_location_id: cmd.primary_location_id,
            updated_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::Updated(event)])
    }

    fn handle_change_status(&mut self, cmd: ChangeOrganizationStatus) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if transition is valid
        if !self.status.can_transition_to(&cmd.new_status) {
            return Err(OrganizationError::InvalidStatusTransition(
                format!("Cannot transition from {} to {}", self.status, cmd.new_status)
            ));
        }

        let event = OrganizationStatusChanged {
            organization_id: self.id,
            old_status: self.status,
            new_status: cmd.new_status,
            reason: cmd.reason,
            changed_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::StatusChanged(event)])
    }

    fn handle_add_member(&mut self, cmd: AddMember) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if organization can have members
        if !self.status.can_have_members() {
            return Err(OrganizationError::InvalidStatus(
                format!("Cannot add members to organization in {} status", self.status)
            ));
        }

        // Check if person is already a member
        if self.members.contains_key(&cmd.person_id) {
            return Err(OrganizationError::MemberAlreadyExists(cmd.person_id));
        }

        // Validate reporting relationship if specified
        if let Some(manager_id) = cmd.reports_to {
            if !self.members.contains_key(&manager_id) {
                return Err(OrganizationError::ManagerNotFound(manager_id));
            }
            if manager_id == cmd.person_id {
                return Err(OrganizationError::InvalidReportingRelationship(
                    "Person cannot report to themselves".to_string()
                ));
            }
        }

        let member = OrganizationMember::new(cmd.person_id, self.id, cmd.role);
        let mut member_with_manager = member.clone();
        if let Some(manager_id) = cmd.reports_to {
            member_with_manager.reports_to = Some(manager_id);
        }

        let event = MemberAdded {
            organization_id: self.id,
            member: member_with_manager,
            added_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::MemberAdded(event)])
    }

    fn handle_remove_member(&mut self, cmd: RemoveMember) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if member exists
        if !self.members.contains_key(&cmd.person_id) {
            return Err(OrganizationError::MemberNotFound(cmd.person_id));
        }

        // Check if anyone reports to this person
        let has_reports = self.members.values()
            .any(|m| m.reports_to == Some(cmd.person_id));
        
        if has_reports {
            return Err(OrganizationError::HasDirectReports(cmd.person_id));
        }

        let event = MemberRemoved {
            organization_id: self.id,
            person_id: cmd.person_id,
            reason: cmd.reason,
            removed_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::MemberRemoved(event)])
    }

    fn handle_update_member_role(&mut self, cmd: UpdateMemberRole) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if member exists
        let member = self.members.get(&cmd.person_id)
            .ok_or(OrganizationError::MemberNotFound(cmd.person_id))?;

        let event = MemberRoleUpdated {
            organization_id: self.id,
            person_id: cmd.person_id,
            old_role: member.role.clone(),
            new_role: cmd.new_role,
            updated_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::MemberRoleUpdated(event)])
    }

    fn handle_change_reporting(&mut self, cmd: ChangeReportingRelationship) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if member exists
        let member = self.members.get(&cmd.person_id)
            .ok_or(OrganizationError::MemberNotFound(cmd.person_id))?;

        // Validate new manager if specified
        if let Some(manager_id) = cmd.new_manager_id {
            if !self.members.contains_key(&manager_id) {
                return Err(OrganizationError::ManagerNotFound(manager_id));
            }
            if manager_id == cmd.person_id {
                return Err(OrganizationError::InvalidReportingRelationship(
                    "Person cannot report to themselves".to_string()
                ));
            }
            
            // Check for circular reporting
            if self.would_create_circular_reporting(cmd.person_id, manager_id) {
                return Err(OrganizationError::InvalidReportingRelationship(
                    "Would create circular reporting relationship".to_string()
                ));
            }
        }

        let event = ReportingRelationshipChanged {
            organization_id: self.id,
            person_id: cmd.person_id,
            old_manager_id: member.reports_to,
            new_manager_id: cmd.new_manager_id,
            changed_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::ReportingRelationshipChanged(event)])
    }

    fn handle_add_child(&mut self, cmd: AddChildOrganization) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Validate not adding self as child
        if cmd.child_id == self.id {
            return Err(OrganizationError::InvalidHierarchy(
                "Organization cannot be its own child".to_string()
            ));
        }

        // Check if already a child
        if self.child_units.contains(&cmd.child_id) {
            return Err(OrganizationError::ChildAlreadyExists(cmd.child_id));
        }

        let event = ChildOrganizationAdded {
            parent_id: self.id,
            child_id: cmd.child_id,
            added_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::ChildOrganizationAdded(event)])
    }

    fn handle_remove_child(&mut self, cmd: RemoveChildOrganization) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if child exists
        if !self.child_units.contains(&cmd.child_id) {
            return Err(OrganizationError::ChildNotFound(cmd.child_id));
        }

        let event = ChildOrganizationRemoved {
            parent_id: self.id,
            child_id: cmd.child_id,
            removed_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::ChildOrganizationRemoved(event)])
    }

    fn handle_add_location(&mut self, cmd: AddLocation) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if already has this location
        if self.locations.contains(&cmd.location_id) {
            return Err(OrganizationError::LocationAlreadyExists(cmd.location_id));
        }

        let is_primary = cmd.make_primary || self.locations.is_empty();

        let event = LocationAdded {
            organization_id: self.id,
            location_id: cmd.location_id,
            is_primary,
            added_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::LocationAdded(event)])
    }

    fn handle_remove_location(&mut self, cmd: RemoveLocation) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if location exists
        if !self.locations.contains(&cmd.location_id) {
            return Err(OrganizationError::LocationNotFound(cmd.location_id));
        }

        let mut events = vec![
            OrganizationEvent::LocationRemoved(LocationRemoved {
                organization_id: self.id,
                location_id: cmd.location_id,
                removed_at: chrono::Utc::now(),
            })
        ];

        // If this was the primary location, clear it
        if self.primary_location_id == Some(cmd.location_id) {
            events.push(OrganizationEvent::PrimaryLocationChanged(PrimaryLocationChanged {
                organization_id: self.id,
                old_location_id: Some(cmd.location_id),
                new_location_id: self.locations.iter().find(|&&id| id != cmd.location_id).copied().unwrap_or_default(),
                changed_at: chrono::Utc::now(),
            }));
        }

        Ok(events)
    }

    fn handle_change_primary_location(&mut self, cmd: ChangePrimaryLocation) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if location exists
        if !self.locations.contains(&cmd.new_location_id) {
            return Err(OrganizationError::LocationNotFound(cmd.new_location_id));
        }

        let event = PrimaryLocationChanged {
            organization_id: self.id,
            old_location_id: self.primary_location_id,
            new_location_id: cmd.new_location_id,
            changed_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::PrimaryLocationChanged(event)])
    }

    fn handle_dissolve(&mut self, cmd: DissolveOrganization) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if can be dissolved
        if !self.status.can_transition_to(&OrganizationStatus::Dissolved) {
            return Err(OrganizationError::InvalidStatus(
                format!("Cannot dissolve organization in {} status", self.status)
            ));
        }

        // Check if has child organizations
        if !self.child_units.is_empty() {
            return Err(OrganizationError::HasChildOrganizations);
        }

        let event = OrganizationDissolved {
            organization_id: self.id,
            reason: cmd.reason,
            member_disposition: cmd.member_disposition,
            dissolved_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::Dissolved(event)])
    }

    fn handle_merge(&mut self, cmd: MergeOrganizations) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Validate not merging with self
        if cmd.source_organization_id == cmd.target_organization_id {
            return Err(OrganizationError::InvalidMerge(
                "Cannot merge organization with itself".to_string()
            ));
        }

        let event = OrganizationMerged {
            source_organization_id: cmd.source_organization_id,
            target_organization_id: cmd.target_organization_id,
            member_disposition: cmd.member_disposition,
            merged_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::Merged(event)])
    }

    fn handle_acquire(&mut self, cmd: AcquireOrganization) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Validate not acquiring self
        if cmd.acquired_organization_id == cmd.acquiring_organization_id {
            return Err(OrganizationError::InvalidAcquisition(
                "Cannot acquire itself".to_string()
            ));
        }

        let event = OrganizationAcquired {
            acquired_organization_id: cmd.acquired_organization_id,
            acquiring_organization_id: cmd.acquiring_organization_id,
            maintains_independence: cmd.maintains_independence,
            acquired_at: chrono::Utc::now(),
        };

        Ok(vec![OrganizationEvent::Acquired(event)])
    }

    // Event application methods

    fn apply_created(&mut self, event: &OrganizationCreated) {
        self.id = event.organization_id;
        self.name = event.name.clone();
        self.org_type = event.org_type;
        self.parent_id = event.parent_id;
        self.primary_location_id = event.primary_location_id;
        self.status = OrganizationStatus::Active;
    }

    fn apply_updated(&mut self, event: &OrganizationUpdated) {
        if let Some(ref name) = event.name {
            self.name = name.clone();
        }
        if let Some(location_id) = event.primary_location_id {
            self.primary_location_id = Some(location_id);
        }
    }

    fn apply_status_changed(&mut self, event: &OrganizationStatusChanged) {
        self.status = event.new_status;
    }

    fn apply_member_added(&mut self, event: &MemberAdded) {
        self.members.insert(event.member.person_id, event.member.clone());
    }

    fn apply_member_removed(&mut self, event: &MemberRemoved) {
        self.members.remove(&event.person_id);
    }

    fn apply_member_role_updated(&mut self, event: &MemberRoleUpdated) {
        if let Some(member) = self.members.get_mut(&event.person_id) {
            member.role = event.new_role.clone();
        }
    }

    fn apply_reporting_changed(&mut self, event: &ReportingRelationshipChanged) {
        if let Some(member) = self.members.get_mut(&event.person_id) {
            member.reports_to = event.new_manager_id;
        }
    }

    fn apply_child_added(&mut self, event: &ChildOrganizationAdded) {
        self.child_units.insert(event.child_id);
    }

    fn apply_child_removed(&mut self, event: &ChildOrganizationRemoved) {
        self.child_units.remove(&event.child_id);
    }

    fn apply_location_added(&mut self, event: &LocationAdded) {
        self.locations.insert(event.location_id);
        if event.is_primary {
            self.primary_location_id = Some(event.location_id);
        }
    }

    fn apply_location_removed(&mut self, event: &LocationRemoved) {
        self.locations.remove(&event.location_id);
    }

    fn apply_primary_location_changed(&mut self, event: &PrimaryLocationChanged) {
        self.primary_location_id = Some(event.new_location_id);
    }

    fn apply_dissolved(&mut self, _event: &OrganizationDissolved) {
        self.status = OrganizationStatus::Dissolved;
    }

    fn apply_merged(&mut self, _event: &OrganizationMerged) {
        self.status = OrganizationStatus::Merged;
    }

    fn apply_acquired(&mut self, _event: &OrganizationAcquired) {
        self.status = OrganizationStatus::Acquired;
    }

    // Helper methods

    fn would_create_circular_reporting(&self, person_id: Uuid, potential_manager_id: Uuid) -> bool {
        let mut current = potential_manager_id;
        let mut visited = HashSet::new();

        while let Some(member) = self.members.get(&current) {
            if !visited.insert(current) {
                // We've seen this person before - there's already a cycle
                return true;
            }

            if current == person_id {
                // Would create a cycle
                return true;
            }

            match member.reports_to {
                Some(manager_id) => current = manager_id,
                None => break,
            }
        }

        false
    }
}

/// Commands that can be handled by the organization aggregate
#[derive(Debug, Clone)]
pub enum OrganizationCommand {
    Create(CreateOrganization),
    Update(UpdateOrganization),
    ChangeStatus(ChangeOrganizationStatus),
    AddMember(AddMember),
    RemoveMember(RemoveMember),
    UpdateMemberRole(UpdateMemberRole),
    ChangeReportingRelationship(ChangeReportingRelationship),
    AddChildOrganization(AddChildOrganization),
    RemoveChildOrganization(RemoveChildOrganization),
    AddLocation(AddLocation),
    RemoveLocation(RemoveLocation),
    ChangePrimaryLocation(ChangePrimaryLocation),
    Dissolve(DissolveOrganization),
    Merge(MergeOrganizations),
    Acquire(AcquireOrganization),
}

/// Organization events
#[derive(Debug, Clone)]
pub enum OrganizationEvent {
    Created(OrganizationCreated),
    Updated(OrganizationUpdated),
    StatusChanged(OrganizationStatusChanged),
    MemberAdded(MemberAdded),
    MemberRemoved(MemberRemoved),
    MemberRoleUpdated(MemberRoleUpdated),
    ReportingRelationshipChanged(ReportingRelationshipChanged),
    ChildOrganizationAdded(ChildOrganizationAdded),
    ChildOrganizationRemoved(ChildOrganizationRemoved),
    LocationAdded(LocationAdded),
    LocationRemoved(LocationRemoved),
    PrimaryLocationChanged(PrimaryLocationChanged),
    Dissolved(OrganizationDissolved),
    Merged(OrganizationMerged),
    Acquired(OrganizationAcquired),
}

/// Errors that can occur in the organization domain
#[derive(Debug, Clone, thiserror::Error)]
pub enum OrganizationError {
    #[error("Organization not found: {0}")]
    NotFound(Uuid),
    
    #[error("Organization already exists: {0}")]
    AlreadyExists(Uuid),

    #[error("Invalid organization name: {0}")]
    InvalidName(String),

    #[error("Invalid status: {0}")]
    InvalidStatus(String),

    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String),

    #[error("Member already exists: {0}")]
    MemberAlreadyExists(Uuid),

    #[error("Member not found: {0}")]
    MemberNotFound(Uuid),

    #[error("Manager not found: {0}")]
    ManagerNotFound(Uuid),

    #[error("Invalid reporting relationship: {0}")]
    InvalidReportingRelationship(String),

    #[error("Person has direct reports: {0}")]
    HasDirectReports(Uuid),

    #[error("Invalid hierarchy: {0}")]
    InvalidHierarchy(String),

    #[error("Child organization already exists: {0}")]
    ChildAlreadyExists(Uuid),

    #[error("Child organization not found: {0}")]
    ChildNotFound(Uuid),

    #[error("Location already exists: {0}")]
    LocationAlreadyExists(Uuid),

    #[error("Location not found: {0}")]
    LocationNotFound(Uuid),

    #[error("Organization has child organizations")]
    HasChildOrganizations,

    #[error("Invalid merge: {0}")]
    InvalidMerge(String),

    #[error("Invalid acquisition: {0}")]
    InvalidAcquisition(String),
    
    #[error("Cross-domain error: {0}")]
    CrossDomainError(String),
}

/// Repository for organizations
pub struct OrganizationRepository;

impl OrganizationRepository {
    /// Load an organization by ID
    pub async fn load(&self, _id: OrganizationId) -> cim_domain::DomainResult<Option<OrganizationAggregate>> {
        // Implementation would load from event store
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_organization() {
        let mut org = OrganizationAggregate::new(
            Uuid::new_v4(),
            "Test Corp".to_string(),
            OrganizationType::Company,
        );

        let cmd = CreateOrganization {
            organization_id: org.id,
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        };

        let events = org.handle_command(OrganizationCommand::Create(cmd)).unwrap();
        assert_eq!(events.len(), 1);

        if let OrganizationEvent::Created(event) = &events[0] {
            assert_eq!(event.name, "Test Corp");
            assert_eq!(event.org_type, OrganizationType::Company);
        } else {
            panic!("Expected Created event");
        }
    }

    #[test]
    fn test_add_member() {
        let mut org = OrganizationAggregate::new(
            Uuid::new_v4(),
            "Test Corp".to_string(),
            OrganizationType::Company,
        );
        org.status = OrganizationStatus::Active;

        let person_id = Uuid::new_v4();
        let role = OrganizationRole::software_engineer();

        let cmd = AddMember {
            organization_id: org.id,
            person_id,
            role,
            reports_to: None,
        };

        let events = org.handle_command(OrganizationCommand::AddMember(cmd)).unwrap();
        assert_eq!(events.len(), 1);

        org.apply_event(&events[0]).unwrap();
        assert_eq!(org.members.len(), 1);
        assert!(org.members.contains_key(&person_id));
    }

    #[test]
    fn test_circular_reporting_detection() {
        let mut org = OrganizationAggregate::new(
            Uuid::new_v4(),
            "Test Corp".to_string(),
            OrganizationType::Company,
        );
        org.status = OrganizationStatus::Active;

        // Add three people
        let person_a = Uuid::new_v4();
        let person_b = Uuid::new_v4();
        let person_c = Uuid::new_v4();

        // Add them to the organization
        for person_id in [person_a, person_b, person_c] {
            let member = OrganizationMember::new(
                person_id,
                org.id,
                OrganizationRole::software_engineer(),
            );
            org.members.insert(person_id, member);
        }

        // Set up reporting: A -> B -> C
        org.members.get_mut(&person_a).unwrap().reports_to = Some(person_b);
        org.members.get_mut(&person_b).unwrap().reports_to = Some(person_c);

        // Try to make C report to A (would create cycle)
        assert!(org.would_create_circular_reporting(person_c, person_a));

        // Check that valid reporting is allowed
        assert!(!org.would_create_circular_reporting(person_c, Uuid::new_v4()));
    }
} 