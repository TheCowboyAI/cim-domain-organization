//! NATS integration for Organization domain

use async_nats::{Client, jetstream};
use cim_domain::{DomainResult, Command};
use std::sync::Arc;
use futures::StreamExt;
use tracing::{info, error, warn};
use uuid::Uuid;

use crate::aggregate::OrganizationAggregate;
use crate::events::OrganizationEvent;
use crate::commands::OrganizationCommand;
use crate::OrganizationError;
use super::persistence::OrganizationRepository;

/// NATS subject patterns for Organization domain
pub struct OrganizationSubjects;

impl OrganizationSubjects {
    /// Commands subject pattern
    pub fn commands() -> &'static str {
        "organization.commands.>"
    }

    /// Events subject pattern
    pub fn events() -> &'static str {
        "organization.events.>"
    }

    /// Command subject for specific aggregate
    pub fn command_for(aggregate_id: Uuid) -> String {
        format!("organization.commands.{aggregate_id}")
    }

    /// Event subject for specific event type
    pub fn event_for(aggregate_id: Uuid, event_type: &str) -> String {
        format!("organization.events.{aggregate_id}.{event_type}")
    }
}

/// NATS-based event store implementation
pub struct NatsEventStore {
    _client: Client,
    jetstream: jetstream::Context,
    stream_name: String,
}

impl NatsEventStore {
    /// Create a new NATS event store
    pub async fn new(client: Client, stream_name: String) -> DomainResult<Self> {
        let jetstream = jetstream::new(client.clone());

        // Create or update the stream
        let stream_config = jetstream::stream::Config {
            name: stream_name.clone(),
            subjects: vec![OrganizationSubjects::events().to_string()],
            retention: jetstream::stream::RetentionPolicy::Limits,
            storage: jetstream::stream::StorageType::File,
            max_age: std::time::Duration::from_secs(365 * 24 * 60 * 60), // 1 year
            ..Default::default()
        };

        // Try to get existing stream first, create if it doesn't exist
        match jetstream.get_stream(&stream_name).await {
            Ok(_stream) => {
                info!("Using existing JetStream stream: {}", stream_name);
            }
            Err(_) => {
                // Stream doesn't exist, try to create it
                match jetstream.create_stream(stream_config).await {
                    Ok(_) => {
                        info!("Created new JetStream stream: {}", stream_name);
                    },
                    Err(e) => {
                        // Check if error is due to overlapping subjects
                        let error_str = format!("{:?}", e);
                        if error_str.contains("10065") || error_str.contains("overlap") {
                            // Stream exists with same subjects, try to get it
                            jetstream.get_stream(&stream_name).await
                                .map_err(|e2| cim_domain::DomainError::ExternalServiceError {
                                    service: "NATS JetStream".to_string(),
                                    message: format!("Stream with overlapping subjects exists but cannot access it: {e2}"),
                                })?;
                            warn!("Stream already exists with overlapping subjects: {}", stream_name);
                        } else {
                            return Err(cim_domain::DomainError::ExternalServiceError {
                                service: "NATS JetStream".to_string(),
                                message: format!("Failed to create stream: {e}"),
                            });
                        }
                    }
                }
            }
        }

        Ok(Self {
            _client: client,
            jetstream,
            stream_name,
        })
    }

    /// Append events to the stream
    pub async fn append_events(
        &self,
        aggregate_id: Uuid,
        events: Vec<OrganizationEvent>,
    ) -> DomainResult<()> {
        // Publish each event
        for event in events {
            let event_type = match &event {
                OrganizationEvent::OrganizationCreated(_) => "created",
                OrganizationEvent::OrganizationUpdated(_) => "updated",
                OrganizationEvent::DepartmentCreated(_) => "department_created",
                OrganizationEvent::TeamFormed(_) => "team_formed",
                OrganizationEvent::RoleCreated(_) => "role_created",
                OrganizationEvent::MemberAdded(_) => "member_added",
                OrganizationEvent::MemberRoleUpdated(_) => "member_role_updated",
                OrganizationEvent::MemberRemoved(_) => "member_removed",
                OrganizationEvent::LocationAdded(_) => "location_added",
                OrganizationEvent::PrimaryLocationChanged(_) => "location_changed",
                OrganizationEvent::LocationRemoved(_) => "location_removed",
                OrganizationEvent::OrganizationStatusChanged(_) => "status_changed",
                OrganizationEvent::ReportingRelationshipChanged(_) => "reporting_changed",
                OrganizationEvent::ChildOrganizationAdded(_) => "child_added",
                OrganizationEvent::ChildOrganizationRemoved(_) => "child_removed",
                OrganizationEvent::OrganizationDissolved(_) => "dissolved",
                OrganizationEvent::OrganizationMerged(_) => "merged",
                _ => "unknown",
            };

            let subject = OrganizationSubjects::event_for(aggregate_id, event_type);

            let payload = serde_json::to_vec(&event)
                .map_err(|e| cim_domain::DomainError::SerializationError(e.to_string()))?;

            self.jetstream
                .publish(subject, payload.into())
                .await
                .map_err(|e| cim_domain::DomainError::ExternalServiceError {
                    service: "NATS JetStream".to_string(),
                    message: format!("Failed to publish event: {e}"),
                })?;
        }

        Ok(())
    }
}

/// Command handler for Organization domain
pub struct OrganizationCommandHandler {
    repository: Arc<OrganizationRepository>,
    client: Client,
}

impl OrganizationCommandHandler {
    /// Create a new command handler
    pub fn new(repository: Arc<OrganizationRepository>, client: Client) -> Self {
        Self { repository, client }
    }

    /// Start listening for commands
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut subscriber = self.client
            .subscribe(OrganizationSubjects::commands().to_string())
            .await?;

        info!("Listening for commands on: {}", OrganizationSubjects::commands());

        while let Some(message) = subscriber.next().await {
            match serde_json::from_slice::<OrganizationCommand>(&message.payload) {
                Ok(command) => {
                    info!("Received command: {:?}", std::any::type_name_of_val(&command));

                    if let Err(e) = self.handle_command(command).await {
                        error!("Failed to handle command: {}", e);

                        // Respond with error if reply subject exists
                        if let Some(reply) = message.reply {
                            let error_response = serde_json::json!({
                                "error": format!("{}", e)
                            });
                            if let Ok(payload) = serde_json::to_vec(&error_response) {
                                let _ = self.client.publish(reply, payload.into()).await;
                            }
                        }
                    } else {
                        // Respond with success if reply subject exists
                        if let Some(reply) = message.reply {
                            let success_response = serde_json::json!({
                                "status": "ok"
                            });
                            if let Ok(payload) = serde_json::to_vec(&success_response) {
                                let _ = self.client.publish(reply, payload.into()).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to deserialize command: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Handle a single command
    async fn handle_command(&self, command: OrganizationCommand) -> Result<(), OrganizationError> {
        // Get aggregate ID from command
        let aggregate_id = command.aggregate_id()
            .map(|id| id.into())
            .unwrap_or_else(|| Uuid::now_v7());

        // Get or create aggregate
        let mut aggregate = self.repository
            .get(aggregate_id)
            .await
            .unwrap_or_else(|_| OrganizationAggregate::new(
                aggregate_id,
                "New Organization".to_string(),
                crate::entity::OrganizationType::Corporation,
            ));

        // Handle command
        let events = aggregate.handle_command(command)?;

        // Save events
        self.repository.save(aggregate_id, events).await?;

        Ok(())
    }
}
