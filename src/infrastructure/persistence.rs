//! Persistence layer for Organization domain

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::aggregate::OrganizationAggregate;
use crate::events::OrganizationEvent;
use crate::OrganizationResult;
use super::nats_integration::NatsEventStore;

/// Snapshot for OrganizationAggregate
#[derive(Clone, Debug)]
pub struct OrganizationSnapshot {
    pub aggregate: OrganizationAggregate,
    pub version: u64,
}

/// In-memory snapshot store
pub struct InMemorySnapshotStore {
    snapshots: RwLock<HashMap<Uuid, OrganizationSnapshot>>,
}

impl InMemorySnapshotStore {
    pub fn new() -> Self {
        Self {
            snapshots: RwLock::new(HashMap::new()),
        }
    }

    pub fn save(&self, aggregate_id: Uuid, snapshot: OrganizationSnapshot) -> OrganizationResult<()> {
        let mut snapshots = self.snapshots.write().unwrap();
        snapshots.insert(aggregate_id, snapshot);
        Ok(())
    }

    pub fn get(&self, aggregate_id: Uuid) -> Option<OrganizationSnapshot> {
        let snapshots = self.snapshots.read().unwrap();
        snapshots.get(&aggregate_id).cloned()
    }
}

impl Default for InMemorySnapshotStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Repository for OrganizationAggregate
pub struct OrganizationRepository {
    event_store: Arc<NatsEventStore>,
    snapshot_store: Arc<InMemorySnapshotStore>,
    snapshot_frequency: u64,
}

impl OrganizationRepository {
    pub fn new(
        event_store: Arc<NatsEventStore>,
        snapshot_store: Arc<InMemorySnapshotStore>,
        snapshot_frequency: u64,
    ) -> Self {
        Self {
            event_store,
            snapshot_store,
            snapshot_frequency,
        }
    }

    /// Get aggregate by ID, rebuilding from events if necessary
    pub async fn get(&self, aggregate_id: Uuid) -> OrganizationResult<OrganizationAggregate> {
        // Try to load from snapshot first
        if let Some(snapshot) = self.snapshot_store.get(aggregate_id) {
            // TODO: Load events after snapshot and replay
            return Ok(snapshot.aggregate);
        }

        // No snapshot, would need to replay all events
        // For now, return error - in production this would replay from event store
        Err(crate::OrganizationError::EntityNotFound(
            format!("Organization {} not found", aggregate_id)
        ))
    }

    /// Save events and update aggregate
    pub async fn save(
        &self,
        aggregate_id: Uuid,
        events: Vec<OrganizationEvent>,
    ) -> OrganizationResult<()> {
        if events.is_empty() {
            return Ok(());
        }

        // Append events to event store
        self.event_store
            .append_events(aggregate_id, events.clone())
            .await?;

        // Get current aggregate or create new one
        let mut aggregate = self.get(aggregate_id).await.unwrap_or_else(|_| {
            OrganizationAggregate::new(
                aggregate_id,
                "Organization".to_string(),
                crate::entity::OrganizationType::Corporation,
            )
        });

        // Apply events to aggregate
        for event in &events {
            aggregate.apply_event(event)?;
        }

        // Check if we should create a snapshot
        if aggregate.version % self.snapshot_frequency == 0 {
            let snapshot = OrganizationSnapshot {
                aggregate: aggregate.clone(),
                version: aggregate.version,
            };
            self.snapshot_store.save(aggregate_id, snapshot)?;
        }

        Ok(())
    }
}
