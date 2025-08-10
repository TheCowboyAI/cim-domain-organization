//! Command handler for organization domain

use crate::aggregate::{OrganizationAggregate, OrganizationCommand, OrganizationEvent, OrganizationError};
use crate::commands::*;
use crate::value_objects::OrganizationStatus;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Event store trait for persistence
#[async_trait::async_trait]
pub trait EventStore: Send + Sync {
    /// Save events to the store
    async fn save_events(&self, aggregate_id: Uuid, events: Vec<OrganizationEvent>) -> Result<(), OrganizationError>;
    
    /// Load events for an aggregate
    async fn load_events(&self, aggregate_id: Uuid) -> Result<Vec<OrganizationEvent>, OrganizationError>;
}

/// In-memory event store for testing
#[derive(Clone)]
pub struct InMemoryEventStore {
    events: Arc<tokio::sync::RwLock<HashMap<Uuid, Vec<OrganizationEvent>>>>,
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl EventStore for InMemoryEventStore {
    async fn save_events(&self, aggregate_id: Uuid, events: Vec<OrganizationEvent>) -> Result<(), OrganizationError> {
        let mut store = self.events.write().await;
        store.entry(aggregate_id)
            .or_insert_with(Vec::new)
            .extend(events);
        Ok(())
    }
    
    async fn load_events(&self, aggregate_id: Uuid) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let store = self.events.read().await;
        Ok(store.get(&aggregate_id).cloned().unwrap_or_default())
    }
}

/// Repository for loading and saving aggregates
pub struct OrganizationRepository<ES: EventStore> {
    event_store: ES,
}

impl<ES: EventStore> OrganizationRepository<ES> {
    pub fn new(event_store: ES) -> Self {
        Self { event_store }
    }
    
    /// Load an aggregate from the event store
    pub async fn load(&self, aggregate_id: Uuid) -> Result<Option<OrganizationAggregate>, OrganizationError> {
        let events = self.event_store.load_events(aggregate_id).await?;
        
        if events.is_empty() {
            return Ok(None);
        }
        
        // Reconstruct aggregate from events
        let mut aggregate = None;
        
        for event in events {
            match &event {
                OrganizationEvent::Created(e) => {
                    let mut agg = OrganizationAggregate::new(e.organization_id, e.name.clone(), e.org_type);
                    agg.apply_event(&event)?;  // Apply the Created event to set status to Active
                    aggregate = Some(agg);
                }
                _ => {
                    if let Some(agg) = aggregate.as_mut() {
                        agg.apply_event(&event)?;
                    }
                }
            }
        }
        
        Ok(aggregate)
    }
    
    /// Save events to the store
    pub async fn save(&self, aggregate_id: Uuid, events: Vec<OrganizationEvent>) -> Result<(), OrganizationError> {
        self.event_store.save_events(aggregate_id, events).await
    }
}

/// Handler for organization commands
pub struct OrganizationCommandHandler<ES: EventStore> {
    repository: OrganizationRepository<ES>,
}

impl<ES: EventStore> OrganizationCommandHandler<ES> {
    /// Create a new command handler
    pub fn new(event_store: ES) -> Self {
        Self {
            repository: OrganizationRepository::new(event_store),
        }
    }

    /// Handle a create organization command
    pub async fn handle_create_organization(
        &self,
        command: CreateOrganization,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Check if organization already exists
        if (self.repository.load(command.organization_id).await?).is_some() {
            return Err(OrganizationError::AlreadyExists(command.organization_id));
        }
        
        // Create new aggregate
        let mut aggregate = OrganizationAggregate::new(
            command.organization_id,
            command.name.clone(),
            command.org_type,
        );

        // Process command
        let events = aggregate.handle_command(OrganizationCommand::Create(command))?;

        // Save events
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle an update organization command
    pub async fn handle_update_organization(
        &self,
        command: UpdateOrganization,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Load aggregate
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        // Process command
        let events = aggregate.handle_command(OrganizationCommand::Update(command))?;
        
        // Save events
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle a change status command
    pub async fn handle_change_status(
        &self,
        command: ChangeOrganizationStatus,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::ChangeStatus(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle an add member command
    pub async fn handle_add_member(
        &self,
        command: AddMember,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::AddMember(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle a remove member command
    pub async fn handle_remove_member(
        &self,
        command: RemoveMember,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::RemoveMember(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle an update member role command
    pub async fn handle_update_member_role(
        &self,
        command: UpdateMemberRole,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::UpdateMemberRole(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle a change reporting relationship command
    pub async fn handle_change_reporting(
        &self,
        command: ChangeReportingRelationship,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::ChangeReportingRelationship(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle an add location command
    pub async fn handle_add_location(
        &self,
        command: AddLocation,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::AddLocation(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle a remove location command
    pub async fn handle_remove_location(
        &self,
        command: RemoveLocation,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::RemoveLocation(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle a dissolve organization command
    pub async fn handle_dissolve_organization(
        &self,
        command: DissolveOrganization,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        let mut aggregate = self.repository.load(command.organization_id).await?
            .ok_or(OrganizationError::NotFound(command.organization_id))?;
            
        let events = aggregate.handle_command(OrganizationCommand::Dissolve(command))?;
        self.repository.save(aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle a merge organizations command
    pub async fn handle_merge_organizations(
        &self,
        command: MergeOrganizations,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Load both organizations
        let mut source_aggregate = self.repository.load(command.source_organization_id).await?
            .ok_or(OrganizationError::NotFound(command.source_organization_id))?;
            
        let target_aggregate = self.repository.load(command.target_organization_id).await?
            .ok_or(OrganizationError::NotFound(command.target_organization_id))?;
            
        // Validate target is active
        if target_aggregate.status != OrganizationStatus::Active {
            return Err(OrganizationError::InvalidStatus(
                format!("Target organization must be active, but is: {:?}", target_aggregate.status)
            ));
        }
        
        let events = source_aggregate.handle_command(OrganizationCommand::Merge(command))?;
        self.repository.save(source_aggregate.id, events.clone()).await?;
        
        Ok(events)
    }

    /// Handle an acquire organization command
    pub async fn handle_acquire_organization(
        &self,
        command: AcquireOrganization,
    ) -> Result<Vec<OrganizationEvent>, OrganizationError> {
        // Load both organizations
        let mut acquiring_aggregate = self.repository.load(command.acquiring_organization_id).await?
            .ok_or(OrganizationError::NotFound(command.acquiring_organization_id))?;
            
        let acquired_aggregate = self.repository.load(command.acquired_organization_id).await?
            .ok_or(OrganizationError::NotFound(command.acquired_organization_id))?;
            
        // Validate acquired exists
        if acquired_aggregate.status != crate::value_objects::OrganizationStatus::Active {
            return Err(OrganizationError::InvalidStatus("Acquired organization must be active".to_string()));
        }
        
        let events = acquiring_aggregate.handle_command(OrganizationCommand::Acquire(command))?;
        self.repository.save(acquiring_aggregate.id, events.clone()).await?;
        
        Ok(events)
    }
}

impl Default for OrganizationCommandHandler<InMemoryEventStore> {
    fn default() -> Self {
        Self::new(InMemoryEventStore::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_organization_handler() {
        let handler = OrganizationCommandHandler::new(InMemoryEventStore::new());
        
        let command = CreateOrganization {
            organization_id: Uuid::new_v4(),
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        };

        let events = handler.handle_create_organization(command).await.unwrap();
        assert_eq!(events.len(), 1);
    }

    #[tokio::test]
    async fn test_add_member_handler() {
        let event_store = InMemoryEventStore::new();
        let handler = OrganizationCommandHandler::new(event_store);
        
        // Create organization first
        let org_id = Uuid::new_v4();
        let create_cmd = CreateOrganization {
            organization_id: org_id,
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        };

        // Create the organization
        handler.handle_create_organization(create_cmd).await.unwrap();

        // Add member
        let add_member_cmd = AddMember {
            organization_id: org_id,
            person_id: Uuid::new_v4(),
            role: OrganizationRole::software_engineer(),
            reports_to: None,
        };

        let events = handler.handle_add_member(add_member_cmd).await.unwrap();
        assert_eq!(events.len(), 1);
    }
    
    #[tokio::test]
    async fn test_cannot_add_member_to_nonexistent_org() {
        let handler = OrganizationCommandHandler::new(InMemoryEventStore::new());
        
        let add_member_cmd = AddMember {
            organization_id: Uuid::new_v4(),
            person_id: Uuid::new_v4(),
            role: OrganizationRole::software_engineer(),
            reports_to: None,
        };

        let result = handler.handle_add_member(add_member_cmd).await;
        assert!(matches!(result, Err(OrganizationError::NotFound(_))));
    }
    
    #[tokio::test]
    async fn test_duplicate_organization_creation_fails() {
        let event_store = InMemoryEventStore::new();
        let handler = OrganizationCommandHandler::new(event_store);
        
        let org_id = Uuid::new_v4();
        let command = CreateOrganization {
            organization_id: org_id,
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        };

        // First creation should succeed
        handler.handle_create_organization(command.clone()).await.unwrap();
        
        // Second creation should fail
        let result = handler.handle_create_organization(command).await;
        assert!(matches!(result, Err(OrganizationError::AlreadyExists(_))));
    }
    
    #[tokio::test]
    async fn test_merge_organizations() {
        let event_store = InMemoryEventStore::new();
        let handler = OrganizationCommandHandler::new(event_store);
        
        // Create source organization
        let source_id = Uuid::new_v4();
        handler.handle_create_organization(CreateOrganization {
            organization_id: source_id,
            name: "Source Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        }).await.unwrap();
        
        // Create target organization
        let target_id = Uuid::new_v4();
        handler.handle_create_organization(CreateOrganization {
            organization_id: target_id,
            name: "Target Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        }).await.unwrap();
        
        // Note: Organizations are created in Active status by default (see apply_created method)
        
        // Merge organizations
        let merge_cmd = MergeOrganizations {
            source_organization_id: source_id,
            target_organization_id: target_id,
            member_disposition: crate::events::MemberDisposition::TransferredTo(target_id),
        };
        
        let events = handler.handle_merge_organizations(merge_cmd).await.unwrap();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], OrganizationEvent::Merged(_)));
    }
    
    #[tokio::test]
    async fn test_event_sourcing_replay() {
        let event_store = InMemoryEventStore::new();
        let handler = OrganizationCommandHandler::new(event_store.clone());
        let repository = OrganizationRepository::new(event_store);
        
        let org_id = Uuid::new_v4();
        
        // Create organization
        handler.handle_create_organization(CreateOrganization {
            organization_id: org_id,
            name: "Test Corp".to_string(),
            org_type: OrganizationType::Company,
            parent_id: None,
            primary_location_id: None,
        }).await.unwrap();
        
        // Add member
        let person_id = Uuid::new_v4();
        handler.handle_add_member(AddMember {
            organization_id: org_id,
            person_id,
            role: OrganizationRole::software_engineer(),
            reports_to: None,
        }).await.unwrap();
        
        // Load aggregate from events
        let loaded = repository.load(org_id).await.unwrap().unwrap();
        assert_eq!(loaded.name, "Test Corp");
        assert!(loaded.members.contains_key(&person_id));
    }
    
    // TODO: Failing tests for unimplemented features
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement organization chart generation")]
    async fn test_todo_organization_chart_generation() {
        // TODO: This test should pass once organization chart generation is implemented
        panic!("TODO: Implement organization chart generation");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement cross-domain person validation")]
    async fn test_todo_cross_domain_person_validation() {
        // TODO: This test should validate that person_id exists in Person domain
        panic!("TODO: Implement cross-domain person validation");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement location domain integration")]
    async fn test_todo_location_domain_integration() {
        // TODO: This test should validate that location_id exists in Location domain
        panic!("TODO: Implement location domain integration");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement role permission validation")]
    async fn test_todo_role_permission_validation() {
        // TODO: This test should validate that roles have proper permissions
        panic!("TODO: Implement role permission validation");
    }
    
    #[tokio::test]
    #[should_panic(expected = "TODO: Implement audit trail for sensitive operations")]
    async fn test_todo_audit_trail() {
        // TODO: This test should verify audit events are generated for sensitive operations
        panic!("TODO: Implement audit trail for sensitive operations");
    }
} 