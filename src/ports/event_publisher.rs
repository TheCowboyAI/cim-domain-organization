//! Event publisher port for NATS JetStream
//!
//! This port defines the interface for publishing events to NATS JetStream.
//! The actual implementation (adapter) would be injected at runtime.

use async_trait::async_trait;
use crate::OrganizationEvent;
use cim_domain::DomainEvent;
use uuid::Uuid;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Publish an organization event to NATS JetStream
    async fn publish(&self, event: &OrganizationEvent) -> Result<(), PublishError>;

    /// Publish multiple events as a batch
    async fn publish_batch(&self, events: &[OrganizationEvent]) -> Result<(), PublishError>;

    /// Query events by correlation ID from JetStream
    async fn query_by_correlation(&self, correlation_id: Uuid) -> Result<Vec<OrganizationEvent>, QueryError>;

    /// Query events by aggregate ID from JetStream
    async fn query_by_aggregate(&self, aggregate_id: Uuid) -> Result<Vec<OrganizationEvent>, QueryError>;

    /// Query events within a time range
    async fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<OrganizationEvent>, QueryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    #[error("Failed to connect to NATS: {0}")]
    ConnectionError(String),

    #[error("Failed to publish event: {0}")]
    PublishFailed(String),

    #[error("Stream not found: {0}")]
    StreamNotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("Failed to query events: {0}")]
    QueryFailed(String),

    #[error("Consumer error: {0}")]
    ConsumerError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

/// Helper to determine the NATS subject for an event
pub fn event_to_subject(event: &OrganizationEvent) -> String {
    let org_id = event.aggregate_id();

    match event {
        OrganizationEvent::OrganizationCreated(_) => {
            format!("events.organization.{}.created", org_id)
        }
        OrganizationEvent::OrganizationUpdated(_) => {
            format!("events.organization.{}.updated", org_id)
        }
        OrganizationEvent::OrganizationDissolved(_) => {
            format!("events.organization.{}.dissolved", org_id)
        }
        OrganizationEvent::OrganizationMerged(_) => {
            format!("events.organization.{}.merged", org_id)
        }
        OrganizationEvent::DepartmentCreated(_) => {
            format!("events.organization.{}.department.created", org_id)
        }
        OrganizationEvent::DepartmentUpdated(_) => {
            format!("events.organization.{}.department.updated", org_id)
        }
        OrganizationEvent::DepartmentRestructured(_) => {
            format!("events.organization.{}.department.restructured", org_id)
        }
        OrganizationEvent::DepartmentDissolved(_) => {
            format!("events.organization.{}.department.dissolved", org_id)
        }
        OrganizationEvent::TeamFormed(_) => {
            format!("events.organization.{}.team.formed", org_id)
        }
        OrganizationEvent::TeamUpdated(_) => {
            format!("events.organization.{}.team.updated", org_id)
        }
        OrganizationEvent::TeamDisbanded(_) => {
            format!("events.organization.{}.team.disbanded", org_id)
        }
        OrganizationEvent::RoleCreated(_) => {
            format!("events.organization.{}.role.created", org_id)
        }
        OrganizationEvent::RoleUpdated(_) => {
            format!("events.organization.{}.role.updated", org_id)
        }
        OrganizationEvent::RoleDeprecated(_) => {
            format!("events.organization.{}.role.deprecated", org_id)
        }
        OrganizationEvent::FacilityCreated(_) => {
            format!("events.organization.{}.facility.created", org_id)
        }
        OrganizationEvent::FacilityUpdated(_) => {
            format!("events.organization.{}.facility.updated", org_id)
        }
        OrganizationEvent::FacilityRemoved(_) => {
            format!("events.organization.{}.facility.removed", org_id)
        }
        OrganizationEvent::OrganizationStatusChanged(_) => {
            format!("events.organization.{}.status.changed", org_id)
        }
        OrganizationEvent::ChildOrganizationAdded(_) => {
            format!("events.organization.{}.child.added", org_id)
        }
        OrganizationEvent::ChildOrganizationRemoved(_) => {
            format!("events.organization.{}.child.removed", org_id)
        }
    }
}