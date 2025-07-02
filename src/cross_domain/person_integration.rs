//! Person domain integration for Organization domain

use crate::errors::OrganizationError;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

/// Request message for getting person details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonDetailsRequest {
    pub person_id: Uuid,
}

/// Response message for person details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonDetailsResponse {
    pub person: Option<super::PersonDetails>,
}

/// Request message for batch person details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonDetailsBatchRequest {
    pub person_ids: Vec<Uuid>,
}

/// Response message for batch person details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonDetailsBatchResponse {
    pub persons: std::collections::HashMap<Uuid, super::PersonDetails>,
}

/// NATS-based cross-domain resolver for Person domain
pub struct NatsPersonResolver {
    nats_client: Arc<async_nats::Client>,
    timeout: Duration,
}

impl NatsPersonResolver {
    pub fn new(nats_client: Arc<async_nats::Client>) -> Self {
        Self { 
            nats_client,
            timeout: Duration::from_secs(5),
        }
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait]
impl super::CrossDomainResolver for NatsPersonResolver {
    async fn get_person_details(&self, person_id: Uuid) -> Result<Option<super::PersonDetails>, OrganizationError> {
        // Create request
        let request = GetPersonDetailsRequest { person_id };
        let payload = serde_json::to_vec(&request)
            .map_err(|e| OrganizationError::SerializationError(e.to_string()))?;
        
        // Send request-reply to Person domain
        let subject = "people.person.query.v1";
        
        match tokio::time::timeout(
            self.timeout,
            self.nats_client.request(subject, payload.into())
        ).await {
            Ok(Ok(msg)) => {
                let response: GetPersonDetailsResponse = serde_json::from_slice(&msg.payload)
                    .map_err(|e| OrganizationError::DeserializationError(e.to_string()))?;
                Ok(response.person)
            },
            Ok(Err(e)) => {
                // NATS error
                tracing::warn!("NATS error getting person details for {}: {}", person_id, e);
                Ok(None)
            },
            Err(_) => {
                // Timeout
                tracing::warn!("Timeout getting person details for {}", person_id);
                Ok(None)
            }
        }
    }
    
    async fn get_location_details(&self, _location_id: Uuid) -> Result<Option<super::LocationDetails>, OrganizationError> {
        // This resolver only handles Person domain
        Ok(None)
    }
    
    async fn get_person_details_batch(&self, person_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::PersonDetails>, OrganizationError> {
        // Create batch request
        let request = GetPersonDetailsBatchRequest { person_ids };
        let payload = serde_json::to_vec(&request)
            .map_err(|e| OrganizationError::SerializationError(e.to_string()))?;
        
        // Send request-reply to Person domain
        let subject = "people.person.query-batch.v1";
        
        match tokio::time::timeout(
            self.timeout,
            self.nats_client.request(subject, payload.into())
        ).await {
            Ok(Ok(msg)) => {
                let response: GetPersonDetailsBatchResponse = serde_json::from_slice(&msg.payload)
                    .map_err(|e| OrganizationError::DeserializationError(e.to_string()))?;
                Ok(response.persons)
            },
            Ok(Err(e)) => {
                // NATS error
                tracing::warn!("NATS error getting batch person details: {}", e);
                Ok(std::collections::HashMap::new())
            },
            Err(_) => {
                // Timeout
                tracing::warn!("Timeout getting batch person details");
                Ok(std::collections::HashMap::new())
            }
        }
    }
    
    async fn get_location_details_batch(&self, _location_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::LocationDetails>, OrganizationError> {
        // This resolver only handles Person domain
        Ok(std::collections::HashMap::new())
    }
}

/// Event handler for Person domain events
pub struct PersonEventHandler {
    nats_client: Arc<async_nats::Client>,
}

impl PersonEventHandler {
    pub fn new(nats_client: Arc<async_nats::Client>) -> Self {
        Self { nats_client }
    }
    
    /// Subscribe to person domain events
    pub async fn subscribe(&self) -> Result<(), OrganizationError> {
        let mut subscriber = self.nats_client
            .subscribe("people.person.*.v1")
            .await
            .map_err(|e| OrganizationError::NatsError(e.to_string()))?;
            
        // Process events in background
        let handler = self.clone();
        tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Err(e) = handler.handle_event(msg).await {
                    tracing::error!("Error handling person event: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    async fn handle_event(&self, msg: async_nats::Message) -> Result<(), OrganizationError> {
        // Parse subject to determine event type
        let parts: Vec<&str> = msg.subject.split('.').collect();
        if parts.len() < 3 {
            return Ok(()); // Invalid subject format, skip
        }
        
        let event_type = parts[2];
        let event_data: serde_json::Value = serde_json::from_slice(&msg.payload)
            .map_err(|e| OrganizationError::DeserializationError(e.to_string()))?;
        
        match event_type {
            "created" => self.handle_person_created(event_data).await,
            "updated" => self.handle_person_updated(event_data).await,
            _ => Ok(()), // Unknown event type, skip
        }
    }
    
    /// Handle person created event from Person domain
    pub async fn handle_person_created(&self, event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update any cached person information
        tracing::info!("Received person created event: {:?}", event);
        // TODO: Update local projections or caches
        Ok(())
    }
    
    /// Handle person updated event from Person domain
    pub async fn handle_person_updated(&self, event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update cached person information
        tracing::info!("Received person updated event: {:?}", event);
        // TODO: Update local projections or caches
        Ok(())
    }
}

impl Clone for PersonEventHandler {
    fn clone(&self) -> Self {
        Self {
            nats_client: self.nats_client.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::CrossDomainResolver;
    
    #[tokio::test]
    #[ignore] // This test requires a NATS server
    async fn test_nats_person_resolver() {
        // This would need a mock NATS client in production tests
        let client = Arc::new(async_nats::connect("nats://localhost:4222").await.unwrap());
        
        let resolver = NatsPersonResolver::new(client);
        let person_id = Uuid::new_v4();
        
        // This will timeout since there's no Person domain service running
        let details = resolver.get_person_details(person_id).await.unwrap();
        assert!(details.is_none()); // Should return None on timeout
    }
} 