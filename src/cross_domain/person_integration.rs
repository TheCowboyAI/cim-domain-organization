//! Person domain integration for Organization domain

use crate::errors::OrganizationError;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

/// NATS-based cross-domain resolver for Person domain
pub struct NatsPersonResolver {
    #[allow(dead_code)]
    nats_client: Arc<async_nats::Client>,
}

impl NatsPersonResolver {
    pub fn new(nats_client: Arc<async_nats::Client>) -> Self {
        Self { nats_client }
    }
}

#[async_trait]
impl super::CrossDomainResolver for NatsPersonResolver {
    async fn get_person_details(&self, person_id: Uuid) -> Result<Option<super::PersonDetails>, OrganizationError> {
        // In production, this would send a NATS request to Person domain
        // For now, return a mock response
        Ok(Some(super::PersonDetails {
            person_id,
            full_name: format!("Person {}", person_id),
            email: Some(format!("person.{}@example.com", person_id)),
            title: Some("Employee".to_string()),
        }))
    }
    
    async fn get_location_details(&self, _location_id: Uuid) -> Result<Option<super::LocationDetails>, OrganizationError> {
        // This resolver only handles Person domain
        Ok(None)
    }
    
    async fn get_person_details_batch(&self, person_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::PersonDetails>, OrganizationError> {
        let mut result = std::collections::HashMap::new();
        
        // In production, this would send a batch request to Person domain
        for person_id in person_ids {
            result.insert(person_id, super::PersonDetails {
                person_id,
                full_name: format!("Person {}", person_id),
                email: Some(format!("person.{}@example.com", person_id)),
                title: Some("Employee".to_string()),
            });
        }
        
        Ok(result)
    }
    
    async fn get_location_details_batch(&self, _location_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::LocationDetails>, OrganizationError> {
        // This resolver only handles Person domain
        Ok(std::collections::HashMap::new())
    }
}

/// Event handler for Person domain events
pub struct PersonEventHandler;

impl PersonEventHandler {
    /// Handle person created event from Person domain
    pub async fn handle_person_created(&self, _event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update any cached person information
        Ok(())
    }
    
    /// Handle person updated event from Person domain
    pub async fn handle_person_updated(&self, _event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update cached person information
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_nats_person_resolver() {
        // This would need a mock NATS client in production tests
        let client = Arc::new(async_nats::connect("nats://localhost:4222").await.unwrap_or_else(|_| {
            panic!("This test requires a NATS server running on localhost:4222");
        }));
        
        let resolver = NatsPersonResolver::new(client);
        let person_id = Uuid::new_v4();
        
        let details = resolver.get_person_details(person_id).await.unwrap();
        assert!(details.is_some());
        
        let details = details.unwrap();
        assert_eq!(details.person_id, person_id);
        assert_eq!(details.full_name, format!("Person {}", person_id));
    }
} 