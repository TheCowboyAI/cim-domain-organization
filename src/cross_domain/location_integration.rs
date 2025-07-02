//! Location domain integration for Organization domain

use crate::errors::OrganizationError;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

/// Request message for getting location details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocationDetailsRequest {
    pub location_id: Uuid,
}

/// Response message for location details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocationDetailsResponse {
    pub location: Option<super::LocationDetails>,
}

/// Request message for batch location details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocationDetailsBatchRequest {
    pub location_ids: Vec<Uuid>,
}

/// Response message for batch location details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocationDetailsBatchResponse {
    pub locations: std::collections::HashMap<Uuid, super::LocationDetails>,
}

/// NATS-based cross-domain resolver for Location domain
pub struct NatsLocationResolver {
    nats_client: Arc<async_nats::Client>,
    timeout: Duration,
}

impl NatsLocationResolver {
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
impl super::CrossDomainResolver for NatsLocationResolver {
    async fn get_person_details(&self, _person_id: Uuid) -> Result<Option<super::PersonDetails>, OrganizationError> {
        // This resolver only handles Location domain
        Ok(None)
    }
    
    async fn get_location_details(&self, location_id: Uuid) -> Result<Option<super::LocationDetails>, OrganizationError> {
        // Create request
        let request = GetLocationDetailsRequest { location_id };
        let payload = serde_json::to_vec(&request)
            .map_err(|e| OrganizationError::SerializationError(e.to_string()))?;
        
        // Send request-reply to Location domain
        let subject = "locations.location.query.v1";
        
        match tokio::time::timeout(
            self.timeout,
            self.nats_client.request(subject, payload.into())
        ).await {
            Ok(Ok(msg)) => {
                let response: GetLocationDetailsResponse = serde_json::from_slice(&msg.payload)
                    .map_err(|e| OrganizationError::DeserializationError(e.to_string()))?;
                Ok(response.location)
            },
            Ok(Err(e)) => {
                // NATS error
                tracing::warn!("NATS error getting location details for {}: {}", location_id, e);
                Ok(None)
            },
            Err(_) => {
                // Timeout
                tracing::warn!("Timeout getting location details for {}", location_id);
                Ok(None)
            }
        }
    }
    
    async fn get_person_details_batch(&self, _person_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::PersonDetails>, OrganizationError> {
        // This resolver only handles Location domain
        Ok(std::collections::HashMap::new())
    }
    
    async fn get_location_details_batch(&self, location_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::LocationDetails>, OrganizationError> {
        // Create batch request
        let request = GetLocationDetailsBatchRequest { location_ids };
        let payload = serde_json::to_vec(&request)
            .map_err(|e| OrganizationError::SerializationError(e.to_string()))?;
        
        // Send request-reply to Location domain
        let subject = "locations.location.query-batch.v1";
        
        match tokio::time::timeout(
            self.timeout,
            self.nats_client.request(subject, payload.into())
        ).await {
            Ok(Ok(msg)) => {
                let response: GetLocationDetailsBatchResponse = serde_json::from_slice(&msg.payload)
                    .map_err(|e| OrganizationError::DeserializationError(e.to_string()))?;
                Ok(response.locations)
            },
            Ok(Err(e)) => {
                // NATS error
                tracing::warn!("NATS error getting batch location details: {}", e);
                Ok(std::collections::HashMap::new())
            },
            Err(_) => {
                // Timeout
                tracing::warn!("Timeout getting batch location details");
                Ok(std::collections::HashMap::new())
            }
        }
    }
}

/// Combined resolver that delegates to both Person and Location domains
pub struct CombinedCrossDomainResolver {
    person_resolver: Arc<dyn super::CrossDomainResolver>,
    location_resolver: Arc<dyn super::CrossDomainResolver>,
}

impl CombinedCrossDomainResolver {
    pub fn new(
        person_resolver: Arc<dyn super::CrossDomainResolver>,
        location_resolver: Arc<dyn super::CrossDomainResolver>,
    ) -> Self {
        Self {
            person_resolver,
            location_resolver,
        }
    }
}

#[async_trait]
impl super::CrossDomainResolver for CombinedCrossDomainResolver {
    async fn get_person_details(&self, person_id: Uuid) -> Result<Option<super::PersonDetails>, OrganizationError> {
        self.person_resolver.get_person_details(person_id).await
    }
    
    async fn get_location_details(&self, location_id: Uuid) -> Result<Option<super::LocationDetails>, OrganizationError> {
        self.location_resolver.get_location_details(location_id).await
    }
    
    async fn get_person_details_batch(&self, person_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::PersonDetails>, OrganizationError> {
        self.person_resolver.get_person_details_batch(person_ids).await
    }
    
    async fn get_location_details_batch(&self, location_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::LocationDetails>, OrganizationError> {
        self.location_resolver.get_location_details_batch(location_ids).await
    }
}

/// Event handler for Location domain events
pub struct LocationEventHandler {
    nats_client: Arc<async_nats::Client>,
}

impl LocationEventHandler {
    pub fn new(nats_client: Arc<async_nats::Client>) -> Self {
        Self { nats_client }
    }
    
    /// Subscribe to location domain events
    pub async fn subscribe(&self) -> Result<(), OrganizationError> {
        let mut subscriber = self.nats_client
            .subscribe("locations.location.*.v1")
            .await
            .map_err(|e| OrganizationError::NatsError(e.to_string()))?;
            
        // Process events in background
        let handler = self.clone();
        tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Err(e) = handler.handle_event(msg).await {
                    tracing::error!("Error handling location event: {}", e);
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
            "created" => self.handle_location_created(event_data).await,
            "updated" => self.handle_location_updated(event_data).await,
            _ => Ok(()), // Unknown event type, skip
        }
    }
    
    /// Handle location created event from Location domain
    pub async fn handle_location_created(&self, event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update any cached location information
        tracing::info!("Received location created event: {:?}", event);
        // TODO: Update local projections or caches
        Ok(())
    }
    
    /// Handle location updated event from Location domain
    pub async fn handle_location_updated(&self, event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update cached location information
        tracing::info!("Received location updated event: {:?}", event);
        // TODO: Update local projections or caches
        Ok(())
    }
}

impl Clone for LocationEventHandler {
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
    async fn test_combined_resolver() {
        // Create mock resolvers
        let person_resolver = Arc::new(super::super::InMemoryCrossDomainResolver::new());
        let location_resolver = Arc::new(super::super::InMemoryCrossDomainResolver::new());
        
        // Add test data
        let person_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();
        
        person_resolver.add_person(super::super::PersonDetails {
            person_id,
            full_name: "Test Person".to_string(),
            email: Some("test@example.com".to_string()),
            title: Some("Manager".to_string()),
        }).await;
        
        location_resolver.add_location(super::super::LocationDetails {
            location_id,
            name: "Test Office".to_string(),
            address: "456 Test St".to_string(),
            city: "Test City".to_string(),
            country: "Test Country".to_string(),
        }).await;
        
        // Create combined resolver
        let combined = CombinedCrossDomainResolver::new(person_resolver, location_resolver);
        
        // Test person resolution
        let person_details = combined.get_person_details(person_id).await.unwrap();
        assert!(person_details.is_some());
        assert_eq!(person_details.unwrap().full_name, "Test Person");
        
        // Test location resolution
        let location_details = combined.get_location_details(location_id).await.unwrap();
        assert!(location_details.is_some());
        assert_eq!(location_details.unwrap().name, "Test Office");
    }
} 