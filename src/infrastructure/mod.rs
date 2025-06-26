//! Infrastructure layer for organization domain

mod component_store;
mod event_store;

pub use component_store::{ComponentStore, InMemoryComponentStore};
pub use event_store::{EventStore, InMemoryEventStore};

// Re-export types that are used across the domain
pub use crate::aggregate::{OrganizationAggregate, OrganizationId, OrganizationRepository}; 