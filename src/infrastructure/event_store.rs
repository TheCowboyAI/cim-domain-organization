//! Event store infrastructure for organization domain

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use cim_domain::DomainResult;
use crate::events::ComponentDataEvent;

/// Trait for storing domain events
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Store an event
    async fn append(&self, event: ComponentDataEvent) -> DomainResult<()>;
    
    /// Get all events
    async fn get_events(&self) -> DomainResult<Vec<ComponentDataEvent>>;
}

/// In-memory implementation of event store
pub struct InMemoryEventStore {
    events: Arc<RwLock<Vec<ComponentDataEvent>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append(&self, event: ComponentDataEvent) -> DomainResult<()> {
        let mut events = self.events.write().await;
        events.push(event);
        Ok(())
    }
    
    async fn get_events(&self) -> DomainResult<Vec<ComponentDataEvent>> {
        let events = self.events.read().await;
        Ok(events.clone())
    }
} 