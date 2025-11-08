//! NATS JetStream implementation of EventPublisher
//!
//! This adapter implements the EventPublisher port using NATS JetStream
//! for persistent event storage and querying.

use async_trait::async_trait;
use async_nats::jetstream::{self, stream::Config as StreamConfig};
use crate::ports::event_publisher::{EventPublisher, PublishError, QueryError, event_to_subject};
use crate::OrganizationEvent;
use cim_domain::DomainEvent;
use futures::StreamExt;
use uuid::Uuid;

pub struct NatsEventPublisher {
    client: async_nats::Client,
    jetstream: jetstream::Context,
    stream_name: String,
}

impl NatsEventPublisher {
    /// Create a new NATS event publisher
    pub async fn new(nats_url: &str, stream_name: &str) -> Result<Self, PublishError> {
        let client = async_nats::connect(nats_url)
            .await
            .map_err(|e| PublishError::ConnectionError(e.to_string()))?;

        let jetstream = jetstream::new(client.clone());

        // Create or get the stream for organization events
        let stream_config = StreamConfig {
            name: stream_name.to_string(),
            subjects: vec![
                "events.organization.>".to_string(),
            ],
            retention: jetstream::stream::RetentionPolicy::Limits,
            storage: jetstream::stream::StorageType::File,
            max_messages: 1_000_000, // Keep last 1M events
            ..Default::default()
        };

        jetstream
            .create_stream(stream_config)
            .await
            .map_err(|e| PublishError::StreamNotFound(e.to_string()))?;

        Ok(Self {
            client,
            jetstream,
            stream_name: stream_name.to_string(),
        })
    }
}

#[async_trait]
impl EventPublisher for NatsEventPublisher {
    async fn publish(&self, event: &OrganizationEvent) -> Result<(), PublishError> {
        let subject = event_to_subject(event);

        // Serialize event
        let payload = serde_json::to_vec(event)
            .map_err(|e| PublishError::SerializationError(e.to_string()))?;

        // Extract correlation ID for header
        let correlation_id = match event {
            OrganizationEvent::MemberAdded(e) => &e.identity.correlation_id,
            OrganizationEvent::MemberRoleUpdated(e) => &e.identity.correlation_id,
            OrganizationEvent::MemberRemoved(e) => &e.identity.correlation_id,
            OrganizationEvent::ReportingRelationshipChanged(e) => &e.identity.correlation_id,
            OrganizationEvent::OrganizationCreated(e) => &e.identity.correlation_id,
            OrganizationEvent::OrganizationUpdated(e) => &e.identity.correlation_id,
            OrganizationEvent::OrganizationDissolved(e) => &e.identity.correlation_id,
            OrganizationEvent::OrganizationMerged(e) => &e.identity.correlation_id,
            OrganizationEvent::DepartmentCreated(e) => &e.identity.correlation_id,
            OrganizationEvent::DepartmentUpdated(e) => &e.identity.correlation_id,
            OrganizationEvent::DepartmentRestructured(e) => &e.identity.correlation_id,
            OrganizationEvent::DepartmentDissolved(e) => &e.identity.correlation_id,
            OrganizationEvent::TeamFormed(e) => &e.identity.correlation_id,
            OrganizationEvent::TeamUpdated(e) => &e.identity.correlation_id,
            OrganizationEvent::TeamDisbanded(e) => &e.identity.correlation_id,
            OrganizationEvent::RoleCreated(e) => &e.identity.correlation_id,
            OrganizationEvent::RoleUpdated(e) => &e.identity.correlation_id,
            OrganizationEvent::RoleAssigned(e) => &e.identity.correlation_id,
            OrganizationEvent::RoleVacated(e) => &e.identity.correlation_id,
            OrganizationEvent::RoleDeprecated(e) => &e.identity.correlation_id,
            OrganizationEvent::LocationAdded(e) => &e.identity.correlation_id,
            OrganizationEvent::PrimaryLocationChanged(e) => &e.identity.correlation_id,
            OrganizationEvent::LocationRemoved(e) => &e.identity.correlation_id,
            OrganizationEvent::OrganizationStatusChanged(e) => &e.identity.correlation_id,
            OrganizationEvent::ChildOrganizationAdded(e) => &e.identity.correlation_id,
            OrganizationEvent::ChildOrganizationRemoved(e) => &e.identity.correlation_id,
        };

        // Add correlation ID as header for efficient querying
        let mut headers = async_nats::HeaderMap::new();

        // Extract correlation ID
        let corr_id_str = match correlation_id {
            cim_domain::CorrelationId::Single(id) => id.to_string(),
            cim_domain::CorrelationId::Transaction(id) => id.0.to_string(), // Access inner UUID
        };

        headers.insert("X-Correlation-ID", corr_id_str.as_str());
        headers.insert("X-Aggregate-ID", event.aggregate_id().to_string().as_str());
        headers.insert("X-Event-Type", event.event_type());

        // Publish to JetStream with headers
        self.jetstream
            .publish_with_headers(subject, headers, payload.into())
            .await
            .map_err(|e| PublishError::PublishFailed(e.to_string()))?
            .await
            .map_err(|e| PublishError::PublishFailed(e.to_string()))?;

        Ok(())
    }

    async fn publish_batch(&self, events: &[OrganizationEvent]) -> Result<(), PublishError> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }

    async fn query_by_correlation(&self, correlation_id: Uuid) -> Result<Vec<OrganizationEvent>, QueryError> {
        let stream = self.jetstream
            .get_stream(&self.stream_name)
            .await
            .map_err(|e| QueryError::QueryFailed(e.to_string()))?;

        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                name: Some(format!("corr_{}", correlation_id)),
                ..Default::default()
            })
            .await
            .map_err(|e| QueryError::ConsumerError(e.to_string()))?;

        let mut messages = consumer
            .fetch()
            .max_messages(1000)
            .messages()
            .await
            .map_err(|e| QueryError::QueryFailed(e.to_string()))?;

        let mut events = Vec::new();

        while let Some(msg) = messages.next().await {
            let msg = msg.map_err(|e| QueryError::QueryFailed(e.to_string()))?;

            // Check correlation ID in header
            if let Some(headers) = &msg.headers {
                if let Some(corr_id) = headers.get("X-Correlation-ID") {
                    if corr_id.as_str() == correlation_id.to_string() {
                        let event: OrganizationEvent = serde_json::from_slice(&msg.payload)
                            .map_err(|e| QueryError::DeserializationError(e.to_string()))?;
                        events.push(event);
                    }
                }
            }

            msg.ack().await.map_err(|e| QueryError::QueryFailed(e.to_string()))?;
        }

        Ok(events)
    }

    async fn query_by_aggregate(&self, aggregate_id: Uuid) -> Result<Vec<OrganizationEvent>, QueryError> {
        let stream = self.jetstream
            .get_stream(&self.stream_name)
            .await
            .map_err(|e| QueryError::QueryFailed(e.to_string()))?;

        // Create consumer filtered by aggregate subject pattern
        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                name: Some(format!("agg_{}", aggregate_id)),
                filter_subject: format!("events.organization.{}.>", aggregate_id),
                ..Default::default()
            })
            .await
            .map_err(|e| QueryError::ConsumerError(e.to_string()))?;

        let mut messages = consumer
            .fetch()
            .max_messages(1000)
            .messages()
            .await
            .map_err(|e| QueryError::QueryFailed(e.to_string()))?;

        let mut events = Vec::new();

        while let Some(msg) = messages.next().await {
            let msg = msg.map_err(|e| QueryError::QueryFailed(e.to_string()))?;

            let event: OrganizationEvent = serde_json::from_slice(&msg.payload)
                .map_err(|e| QueryError::DeserializationError(e.to_string()))?;

            events.push(event);

            msg.ack().await.map_err(|e| QueryError::QueryFailed(e.to_string()))?;
        }

        Ok(events)
    }

    async fn query_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<OrganizationEvent>, QueryError> {
        let stream = self.jetstream
            .get_stream(&self.stream_name)
            .await
            .map_err(|e| QueryError::QueryFailed(e.to_string()))?;

        let consumer = stream
            .create_consumer(jetstream::consumer::pull::Config {
                name: Some(format!("time_range_{}", Uuid::now_v7())),
                ..Default::default()
            })
            .await
            .map_err(|e| QueryError::ConsumerError(e.to_string()))?;

        let mut messages = consumer
            .fetch()
            .max_messages(10000)
            .messages()
            .await
            .map_err(|e| QueryError::QueryFailed(e.to_string()))?;

        let mut events = Vec::new();

        while let Some(msg) = messages.next().await {
            let msg = msg.map_err(|e| QueryError::QueryFailed(e.to_string()))?;

            let event: OrganizationEvent = serde_json::from_slice(&msg.payload)
                .map_err(|e| QueryError::DeserializationError(e.to_string()))?;

            // Check timestamp
            let event_time = match &event {
                OrganizationEvent::MemberAdded(e) => e.occurred_at,
                OrganizationEvent::MemberRoleUpdated(e) => e.occurred_at,
                OrganizationEvent::MemberRemoved(e) => e.occurred_at,
                OrganizationEvent::OrganizationCreated(e) => e.occurred_at,
                OrganizationEvent::OrganizationUpdated(e) => e.occurred_at,
                OrganizationEvent::OrganizationDissolved(e) => e.effective_date,
                OrganizationEvent::OrganizationMerged(e) => e.effective_date,
                OrganizationEvent::DepartmentCreated(e) => e.occurred_at,
                OrganizationEvent::LocationAdded(e) => e.occurred_at,
                _ => chrono::Utc::now(), // Fallback
            };

            if event_time >= start && event_time <= end {
                events.push(event);
            }

            msg.ack().await.map_err(|e| QueryError::QueryFailed(e.to_string()))?;
        }

        Ok(events)
    }
}