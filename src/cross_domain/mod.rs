//! Cross-domain integration for Organization domain
//!
//! This module handles integration with other domains in the CIM ecosystem:
//! - Person domain: For member name resolution
//! - Location domain: For location name resolution

use crate::aggregate::OrganizationError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod person_integration;
pub mod location_integration;

pub use person_integration::*;
pub use location_integration::*;

/// Cross-domain query for getting person details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPersonDetails {
    pub person_id: Uuid,
}

/// Person details response from Person domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonDetails {
    pub person_id: Uuid,
    pub full_name: String,
    pub email: Option<String>,
    pub title: Option<String>,
}

/// Cross-domain query for getting location details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLocationDetails {
    pub location_id: Uuid,
}

/// Location details response from Location domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDetails {
    pub location_id: Uuid,
    pub name: String,
    pub address: String,
    pub city: String,
    pub country: String,
}

/// Service for resolving cross-domain references
#[async_trait]
pub trait CrossDomainResolver: Send + Sync {
    /// Get person details from Person domain
    async fn get_person_details(&self, person_id: Uuid) -> Result<Option<PersonDetails>, OrganizationError>;
    
    /// Get location details from Location domain
    async fn get_location_details(&self, location_id: Uuid) -> Result<Option<LocationDetails>, OrganizationError>;
    
    /// Get multiple person details in batch
    async fn get_person_details_batch(&self, person_ids: Vec<Uuid>) -> Result<HashMap<Uuid, PersonDetails>, OrganizationError>;
    
    /// Get multiple location details in batch
    async fn get_location_details_batch(&self, location_ids: Vec<Uuid>) -> Result<HashMap<Uuid, LocationDetails>, OrganizationError>;
}

/// In-memory implementation for testing
#[derive(Clone)]
pub struct InMemoryCrossDomainResolver {
    persons: Arc<RwLock<HashMap<Uuid, PersonDetails>>>,
    locations: Arc<RwLock<HashMap<Uuid, LocationDetails>>>,
}

impl Default for InMemoryCrossDomainResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryCrossDomainResolver {
    pub fn new() -> Self {
        Self {
            persons: Arc::new(RwLock::new(HashMap::new())),
            locations: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add test person data
    pub async fn add_person(&self, details: PersonDetails) {
        self.persons.write().await.insert(details.person_id, details);
    }
    
    /// Add test location data
    pub async fn add_location(&self, details: LocationDetails) {
        self.locations.write().await.insert(details.location_id, details);
    }
}

#[async_trait]
impl CrossDomainResolver for InMemoryCrossDomainResolver {
    async fn get_person_details(&self, person_id: Uuid) -> Result<Option<PersonDetails>, OrganizationError> {
        Ok(self.persons.read().await.get(&person_id).cloned())
    }
    
    async fn get_location_details(&self, location_id: Uuid) -> Result<Option<LocationDetails>, OrganizationError> {
        Ok(self.locations.read().await.get(&location_id).cloned())
    }
    
    async fn get_person_details_batch(&self, person_ids: Vec<Uuid>) -> Result<HashMap<Uuid, PersonDetails>, OrganizationError> {
        let persons = self.persons.read().await;
        let mut result = HashMap::new();
        
        for person_id in person_ids {
            if let Some(details) = persons.get(&person_id) {
                result.insert(person_id, details.clone());
            }
        }
        
        Ok(result)
    }
    
    async fn get_location_details_batch(&self, location_ids: Vec<Uuid>) -> Result<HashMap<Uuid, LocationDetails>, OrganizationError> {
        let locations = self.locations.read().await;
        let mut result = HashMap::new();
        
        for location_id in location_ids {
            if let Some(details) = locations.get(&location_id) {
                result.insert(location_id, details.clone());
            }
        }
        
        Ok(result)
    }
}

/// Service for handling cross-domain integration
pub struct CrossDomainIntegrationService<R: CrossDomainResolver> {
    resolver: Arc<R>,
}

impl<R: CrossDomainResolver> CrossDomainIntegrationService<R> {
    pub fn new(resolver: Arc<R>) -> Self {
        Self { resolver }
    }
    
    /// Enrich organization view with person names
    pub async fn enrich_with_person_names(
        &self,
        members: &mut Vec<crate::projections::MemberView>,
    ) -> Result<(), OrganizationError> {
        let person_ids: Vec<Uuid> = members.iter().map(|m| m.person_id).collect();
        let person_details = self.resolver.get_person_details_batch(person_ids).await?;
        
        for member in members.iter_mut() {
            if let Some(details) = person_details.get(&member.person_id) {
                member.person_name = details.full_name.clone();
            }
        }
        
        Ok(())
    }
    
    /// Enrich organization view with location name
    pub async fn enrich_with_location_name(
        &self,
        org: &mut crate::projections::OrganizationView,
        location_id: Uuid,
    ) -> Result<(), OrganizationError> {
        if let Some(details) = self.resolver.get_location_details(location_id).await? {
            org.primary_location_name = Some(format!("{}, {}", details.name, details.city));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_person_name_resolution() {
        let resolver = InMemoryCrossDomainResolver::new();
        let service = CrossDomainIntegrationService::new(Arc::new(resolver.clone()));
        
        // Add test person
        let person_id = Uuid::new_v4();
        resolver.add_person(PersonDetails {
            person_id,
            full_name: "John Doe".to_string(),
            email: Some("john@example.com".to_string()),
            title: Some("Software Engineer".to_string()),
        }).await;
        
        // Create member view
        let mut members = vec![
            crate::projections::MemberView {
                person_id,
                person_name: format!("Person {person_id}"),
                role: crate::value_objects::OrganizationRole::software_engineer(),
                joined_at: chrono::Utc::now(),
                reports_to_id: None,
                reports_to_name: None,
                direct_reports_count: 0,
                is_active: true,
            }
        ];
        
        // Enrich with person names
        service.enrich_with_person_names(&mut members).await.unwrap();
        
        assert_eq!(members[0].person_name, "John Doe");
    }
    
    #[tokio::test]
    async fn test_location_name_resolution() {
        let resolver = InMemoryCrossDomainResolver::new();
        let service = CrossDomainIntegrationService::new(Arc::new(resolver.clone()));
        
        // Add test location
        let location_id = Uuid::new_v4();
        resolver.add_location(LocationDetails {
            location_id,
            name: "Main Office".to_string(),
            address: "123 Main St".to_string(),
            city: "San Francisco".to_string(),
            country: "USA".to_string(),
        }).await;
        
        // Create organization view
        let mut org = crate::projections::OrganizationView {
            organization_id: Uuid::new_v4(),
            name: "Test Corp".to_string(),
            org_type: crate::value_objects::OrganizationType::Company,
            status: crate::value_objects::OrganizationStatus::Active,
            parent_id: None,
            child_units: vec![],
            member_count: 100,
            location_count: 1,
            location_id: Some(location_id),
            primary_location_name: None,
            size_category: crate::value_objects::SizeCategory::Large,
        };
        
        // Enrich with location name
        service.enrich_with_location_name(&mut org, location_id).await.unwrap();
        
        assert_eq!(org.primary_location_name, Some("Main Office, San Francisco".to_string()));
    }
} 