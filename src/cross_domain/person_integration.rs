//! Person domain integration for Organization domain

use crate::aggregate::OrganizationError;
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
            .map_err(|e| OrganizationError::CrossDomainError(format!("Serialization error: {}", e)))?;
        
        // Send request-reply to Person domain
        let subject = "people.person.query.v1";
        
        match tokio::time::timeout(
            self.timeout,
            self.nats_client.request(subject, payload.into())
        ).await {
            Ok(Ok(msg)) => {
                let response: GetPersonDetailsResponse = serde_json::from_slice(&msg.payload)
                    .map_err(|e| OrganizationError::CrossDomainError(format!("Deserialization error: {}", e)))?;
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
    
    // TODO: Location details should be handled by composition with cim-domain-location
    // async fn get_location_details(&self, _location_id: Uuid) -> Result<Option<super::LocationDetails>, OrganizationError> {
    //     // This resolver only handles Person domain
    //     Ok(None)
    // }
    
    async fn get_person_details_batch(&self, person_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::PersonDetails>, OrganizationError> {
        // Create batch request
        let request = GetPersonDetailsBatchRequest { person_ids };
        let payload = serde_json::to_vec(&request)
            .map_err(|e| OrganizationError::CrossDomainError(format!("Serialization error: {}", e)))?;
        
        // Send request-reply to Person domain
        let subject = "people.person.query-batch.v1";
        
        match tokio::time::timeout(
            self.timeout,
            self.nats_client.request(subject, payload.into())
        ).await {
            Ok(Ok(msg)) => {
                let response: GetPersonDetailsBatchResponse = serde_json::from_slice(&msg.payload)
                    .map_err(|e| OrganizationError::CrossDomainError(format!("Deserialization error: {}", e)))?;
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
    
    // TODO: Location details should be handled by composition with cim-domain-location
    // async fn get_location_details_batch(&self, _location_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::LocationDetails>, OrganizationError> {
    //     // This resolver only handles Person domain
    //     Ok(std::collections::HashMap::new())
    // }
}

/// Event handler for Person domain events
pub struct PersonEventHandler {
    nats_client: Arc<async_nats::Client>,
    read_model_store: Arc<dyn super::super::handlers::query_handler::ReadModelStore>,
}

impl PersonEventHandler {
    pub fn new(
        nats_client: Arc<async_nats::Client>, 
        read_model_store: Arc<dyn super::super::handlers::query_handler::ReadModelStore>
    ) -> Self {
        Self { 
            nats_client,
            read_model_store,
        }
    }
    
    /// Subscribe to person domain events
    pub async fn subscribe(&self) -> Result<(), OrganizationError> {
        let mut subscriber = self.nats_client
            .subscribe("people.person.*.v1")
            .await
            .map_err(|e| OrganizationError::CrossDomainError(format!("NATS error: {}", e)))?;
            
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
            .map_err(|e| OrganizationError::CrossDomainError(format!("Deserialization error: {}", e)))?;
        
        match event_type {
            "created" => self.handle_person_created(event_data).await,
            "updated" => self.handle_person_updated(event_data).await,
            _ => Ok(()), // Unknown event type, skip
        }
    }
    
    /// Handle person created event from Person domain
    pub async fn handle_person_created(&self, event: serde_json::Value) -> Result<(), OrganizationError> {
        // Extract person details from event
        tracing::info!("Received person created event: {:?}", event);
        
        // Extract person_id and details
        if let Some(person_id) = event.get("person_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()) 
        {
            // Get all organizations this person belongs to
            let person_orgs = self.read_model_store.get_person_organizations(person_id).await?;
            
            // Update member views in each organization
            for member_org in person_orgs {
                let org_members = self.read_model_store.get_organization_members(member_org.organization_id).await?;
                
                // Find and update the member with new person details
                for mut member in org_members {
                    if member.person_id == person_id {
                        // Update person name if available
                        if let Some(full_name) = event.get("full_name").and_then(|v| v.as_str()) {
                            member.person_name = full_name.to_string();
                        }
                        
                        // Update the member view
                        self.read_model_store.update_member(member_org.organization_id, member).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle person updated event from Person domain
    pub async fn handle_person_updated(&self, event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update cached person information
        tracing::info!("Received person updated event: {:?}", event);
        
        // Extract person_id and updated details
        if let Some(person_id) = event.get("person_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()) 
        {
            // Get all organizations this person belongs to
            let person_orgs = self.read_model_store.get_person_organizations(person_id).await?;
            
            // Update member views in each organization
            for member_org in person_orgs {
                let org_members = self.read_model_store.get_organization_members(member_org.organization_id).await?;
                
                // Find and update the member with new person details
                for mut member in org_members {
                    if member.person_id == person_id {
                        // Update person name if available
                        if let Some(full_name) = event.get("full_name").and_then(|v| v.as_str()) {
                            member.person_name = full_name.to_string();
                        }
                        
                        // Update the member view
                        self.read_model_store.update_member(member_org.organization_id, member).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl Clone for PersonEventHandler {
    fn clone(&self) -> Self {
        Self {
            nats_client: self.nats_client.clone(),
            read_model_store: self.read_model_store.clone(),
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