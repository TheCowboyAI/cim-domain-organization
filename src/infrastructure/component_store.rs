//! Component storage infrastructure

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use cim_domain::{DomainResult, DomainError};
use crate::components::data::{ComponentInstance, ComponentInstanceId};
use crate::aggregate::OrganizationId;

/// Trait for storing and retrieving components
#[async_trait]
pub trait ComponentStore: Send + Sync {
    /// Store a component
    async fn store_component<T: Send + Sync + 'static>(&self, component: ComponentInstance<T>) -> DomainResult<()>;
    
    /// Get a component by ID
    async fn get_component<T: Send + Sync + Clone + 'static>(&self, id: ComponentInstanceId) -> DomainResult<Option<ComponentInstance<T>>>;
    
    /// Get all components for an organization
    async fn get_organization_components<T: Send + Sync + Clone + 'static>(&self, organization_id: OrganizationId) -> DomainResult<Vec<ComponentInstance<T>>>;
    
    /// Update a component
    async fn update_component<T: Send + Sync + 'static>(&self, component: ComponentInstance<T>) -> DomainResult<()>;
    
    /// Delete a component
    async fn delete_component(&self, id: ComponentInstanceId) -> DomainResult<()>;
}

/// In-memory implementation of component store
pub struct InMemoryComponentStore {
    storage: Arc<RwLock<HashMap<ComponentInstanceId, Box<dyn std::any::Any + Send + Sync>>>>,
}

impl InMemoryComponentStore {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryComponentStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ComponentStore for InMemoryComponentStore {
    async fn store_component<T: Send + Sync + 'static>(&self, component: ComponentInstance<T>) -> DomainResult<()> {
        let mut storage = self.storage.write().await;
        storage.insert(component.id, Box::new(component));
        Ok(())
    }
    
    async fn get_component<T: Send + Sync + Clone + 'static>(&self, id: ComponentInstanceId) -> DomainResult<Option<ComponentInstance<T>>> {
        let storage = self.storage.read().await;
        
        if let Some(boxed) = storage.get(&id) {
            if let Some(component) = boxed.downcast_ref::<ComponentInstance<T>>() {
                Ok(Some(component.clone()))
            } else {
                Err(DomainError::generic("Component type mismatch"))
            }
        } else {
            Ok(None)
        }
    }
    
    async fn get_organization_components<T: Send + Sync + Clone + 'static>(&self, organization_id: OrganizationId) -> DomainResult<Vec<ComponentInstance<T>>> {
        let storage = self.storage.read().await;
        let mut components = Vec::new();
        
        for (_, boxed) in storage.iter() {
            if let Some(component) = boxed.downcast_ref::<ComponentInstance<T>>() {
                if component.organization_id == organization_id {
                    components.push(component.clone());
                }
            }
        }
        
        Ok(components)
    }
    
    async fn update_component<T: Send + Sync + 'static>(&self, component: ComponentInstance<T>) -> DomainResult<()> {
        let mut storage = self.storage.write().await;
        
        if let std::collections::hash_map::Entry::Occupied(mut e) = storage.entry(component.id) {
            e.insert(Box::new(component));
            Ok(())
        } else {
            Err(DomainError::generic("Component not found"))
        }
    }
    
    async fn delete_component(&self, id: ComponentInstanceId) -> DomainResult<()> {
        let mut storage = self.storage.write().await;
        
        if storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DomainError::generic("Component not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::data::{ContactComponentData, ContactType};
    use crate::value_objects::PhoneNumber;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_store_and_retrieve_component() {
        let store = InMemoryComponentStore::new();
        let org_id = Uuid::new_v4();
        
        let contact_data = ContactComponentData {
            contact_type: ContactType::Main,
            phone: PhoneNumber::new("+1-555-1234".to_string()).unwrap(),
            extension: None,
            department: None,
            hours_of_operation: None,
            is_primary: true,
        };
        
        let component = ComponentInstance::new(org_id, contact_data).unwrap();
        let component_id = component.id;
        
        // Store component
        store.store_component(component.clone()).await.unwrap();
        
        // Retrieve component
        let retrieved: Option<ComponentInstance<ContactComponentData>> = 
            store.get_component(component_id).await.unwrap();
        
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, component_id);
    }
} 