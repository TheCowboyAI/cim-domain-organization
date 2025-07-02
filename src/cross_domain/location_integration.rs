//! Location domain integration for Organization domain

use crate::errors::OrganizationError;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

/// NATS-based cross-domain resolver for Location domain
pub struct NatsLocationResolver {
    #[allow(dead_code)]
    nats_client: Arc<async_nats::Client>,
}

impl NatsLocationResolver {
    pub fn new(nats_client: Arc<async_nats::Client>) -> Self {
        Self { nats_client }
    }
}

#[async_trait]
impl super::CrossDomainResolver for NatsLocationResolver {
    async fn get_person_details(&self, _person_id: Uuid) -> Result<Option<super::PersonDetails>, OrganizationError> {
        // This resolver only handles Location domain
        Ok(None)
    }
    
    async fn get_location_details(&self, location_id: Uuid) -> Result<Option<super::LocationDetails>, OrganizationError> {
        // In production, this would send a NATS request to Location domain
        // For now, return a mock response
        Ok(Some(super::LocationDetails {
            location_id,
            name: "Headquarters".to_string(),
            address: "123 Main Street".to_string(),
            city: "San Francisco".to_string(),
            country: "USA".to_string(),
        }))
    }
    
    async fn get_person_details_batch(&self, _person_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::PersonDetails>, OrganizationError> {
        // This resolver only handles Location domain
        Ok(std::collections::HashMap::new())
    }
    
    async fn get_location_details_batch(&self, location_ids: Vec<Uuid>) -> Result<std::collections::HashMap<Uuid, super::LocationDetails>, OrganizationError> {
        let mut result = std::collections::HashMap::new();
        
        // In production, this would send a batch request to Location domain
        for (i, location_id) in location_ids.into_iter().enumerate() {
            result.insert(location_id, super::LocationDetails {
                location_id,
                name: format!("Location {}", i + 1),
                address: format!("{} Street", i + 1),
                city: "San Francisco".to_string(),
                country: "USA".to_string(),
            });
        }
        
        Ok(result)
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
pub struct LocationEventHandler;

impl LocationEventHandler {
    /// Handle location created event from Location domain
    pub async fn handle_location_created(&self, _event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update any cached location information
        Ok(())
    }
    
    /// Handle location updated event from Location domain
    pub async fn handle_location_updated(&self, _event: serde_json::Value) -> Result<(), OrganizationError> {
        // Update cached location information
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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