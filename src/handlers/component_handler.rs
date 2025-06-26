//! Component command handler for organizations

use cim_domain::{DomainResult, DomainError};
use std::sync::Arc;
use chrono::Utc;

use crate::aggregate::OrganizationId;
use crate::commands::ComponentCommand;
use crate::events::ComponentDataEvent;
use crate::infrastructure::{EventStore, InMemoryComponentStore, OrganizationRepository, ComponentStore};
use crate::components::data::{
    ComponentInstance, ComponentInstanceId, ContactComponentData, AddressComponentData,
    CertificationComponentData, IndustryComponentData, FinancialComponentData,
    SocialMediaComponentData, PartnershipComponentData, ContactType, AddressType,
    CertificationType, CertificationStatus, ClassificationSystem, SocialPlatform,
    PartnershipType,
};
use crate::value_objects::{PhoneNumber, Address};

/// Handler for component commands
pub struct ComponentCommandHandler {
    event_store: Arc<dyn EventStore>,
    component_store: Arc<InMemoryComponentStore>,
    organization_repository: Arc<OrganizationRepository>,
}

impl ComponentCommandHandler {
    pub fn new(
        event_store: Arc<dyn EventStore>,
        component_store: Arc<InMemoryComponentStore>,
        organization_repository: Arc<OrganizationRepository>,
    ) -> Self {
        Self {
            event_store,
            component_store,
            organization_repository,
        }
    }
    
    /// Handle a component command
    pub async fn handle(&self, command: ComponentCommand) -> DomainResult<Vec<ComponentDataEvent>> {
        // Verify organization exists
        let organization_id = self.get_organization_id(&command)?;
        let organization = self.organization_repository.load(organization_id).await?;
        
        if organization.is_none() {
            return Err(DomainError::AggregateNotFound(format!("Organization {}", organization_id)));
        }
        
        // Process command
        match command {
            ComponentCommand::AddContact { organization_id, contact_type, phone_number, extension, department, hours_of_operation, is_primary } => {
                self.handle_add_contact(organization_id, contact_type, phone_number, extension, department, hours_of_operation, is_primary).await
            }
            ComponentCommand::UpdateContact { organization_id, component_id, phone_number, extension, department, hours_of_operation, is_primary } => {
                self.handle_update_contact(organization_id, component_id, phone_number, extension, department, hours_of_operation, is_primary).await
            }
            ComponentCommand::RemoveContact { organization_id, component_id } => {
                self.handle_remove_contact(organization_id, component_id).await
            }
            ComponentCommand::AddAddress { organization_id, address_type, line1, line2, city, state_province, postal_code, country, is_primary, is_billing_address, is_shipping_address } => {
                self.handle_add_address(organization_id, address_type, line1, line2, city, state_province, postal_code, country, is_primary, is_billing_address, is_shipping_address).await
            }
            ComponentCommand::AddCertification { organization_id, certification_type, name, issuing_body, certification_number, issue_date, expiry_date, scope } => {
                self.handle_add_certification(organization_id, certification_type, name, issuing_body, certification_number, issue_date, expiry_date, scope).await
            }
            ComponentCommand::AddIndustry { organization_id, classification_system, code, description, is_primary } => {
                self.handle_add_industry(organization_id, classification_system, code, description, is_primary).await
            }
            ComponentCommand::SetFinancialInfo { organization_id, fiscal_year_end, revenue_range, employee_count_range, credit_rating, duns_number, tax_id } => {
                self.handle_set_financial_info(organization_id, fiscal_year_end, revenue_range, employee_count_range, credit_rating, duns_number, tax_id).await
            }
            ComponentCommand::AddSocialProfile { organization_id, platform, profile_url, handle, is_verified } => {
                self.handle_add_social_profile(organization_id, platform, profile_url, handle, is_verified).await
            }
            ComponentCommand::AddPartnership { organization_id, partner_organization_id, partner_name, partnership_type, start_date, end_date, description } => {
                self.handle_add_partnership(organization_id, partner_organization_id, partner_name, partnership_type, start_date, end_date, description).await
            }
            _ => Err(DomainError::generic("Component command not yet implemented")),
        }
    }
    
    fn get_organization_id(&self, command: &ComponentCommand) -> DomainResult<OrganizationId> {
        match command {
            ComponentCommand::AddContact { organization_id, .. } |
            ComponentCommand::UpdateContact { organization_id, .. } |
            ComponentCommand::RemoveContact { organization_id, .. } |
            ComponentCommand::AddAddress { organization_id, .. } |
            ComponentCommand::UpdateAddress { organization_id, .. } |
            ComponentCommand::RemoveAddress { organization_id, .. } |
            ComponentCommand::AddCertification { organization_id, .. } |
            ComponentCommand::UpdateCertification { organization_id, .. } |
            ComponentCommand::RemoveCertification { organization_id, .. } |
            ComponentCommand::AddIndustry { organization_id, .. } |
            ComponentCommand::UpdateIndustry { organization_id, .. } |
            ComponentCommand::RemoveIndustry { organization_id, .. } |
            ComponentCommand::SetFinancialInfo { organization_id, .. } |
            ComponentCommand::UpdateFinancialInfo { organization_id, .. } |
            ComponentCommand::AddSocialProfile { organization_id, .. } |
            ComponentCommand::UpdateSocialProfile { organization_id, .. } |
            ComponentCommand::RemoveSocialProfile { organization_id, .. } |
            ComponentCommand::AddPartnership { organization_id, .. } |
            ComponentCommand::UpdatePartnership { organization_id, .. } |
            ComponentCommand::RemovePartnership { organization_id, .. } => Ok(*organization_id),
        }
    }
    
    async fn handle_add_contact(
        &self,
        organization_id: OrganizationId,
        contact_type: ContactType,
        phone_number: String,
        extension: Option<String>,
        department: Option<String>,
        hours_of_operation: Option<String>,
        is_primary: bool,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create contact component
        let phone = PhoneNumber::new(phone_number.clone())
            .map_err(|e| DomainError::ValidationError(e))?;
        let contact_data = ContactComponentData {
            contact_type,
            phone,
            extension,
            department,
            hours_of_operation,
            is_primary,
        };
        
        let component = ComponentInstance::new(organization_id, contact_data)?;
        let component_id = component.id;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::ContactAdded {
            organization_id,
            component_id,
            contact_type,
            phone_number,
            is_primary,
            timestamp: Utc::now(),
        };
        
        // Store event
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_update_contact(
        &self,
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        phone_number: Option<String>,
        extension: Option<String>,
        department: Option<String>,
        hours_of_operation: Option<String>,
        is_primary: Option<bool>,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Get existing component
        let existing: Option<ComponentInstance<ContactComponentData>> = 
            self.component_store.get_component(component_id).await?;
        
        if existing.is_none() {
            return Err(DomainError::generic(format!("Contact component {} not found", component_id)));
        }
        
        let mut component = existing.unwrap();
        
        // Apply changes
        if let Some(new_phone) = &phone_number {
            component.data.phone = PhoneNumber::new(new_phone.clone())
                .map_err(|e| DomainError::ValidationError(e))?;
        }
        if let Some(ext) = extension.clone() {
            component.data.extension = Some(ext);
        }
        if let Some(dept) = department.clone() {
            component.data.department = Some(dept);
        }
        if let Some(hours) = hours_of_operation.clone() {
            component.data.hours_of_operation = Some(hours);
        }
        if let Some(primary) = is_primary {
            component.data.is_primary = primary;
        }
        
        // Update component
        self.component_store.update_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::ContactUpdated {
            organization_id,
            component_id,
            changes: crate::events::ContactChanges {
                phone_number,
                extension,
                department,
                hours_of_operation,
                is_primary,
            },
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_remove_contact(
        &self,
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Delete component
        self.component_store.delete_component(component_id).await?;
        
        // Create event
        let event = ComponentDataEvent::ContactRemoved {
            organization_id,
            component_id,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_add_address(
        &self,
        organization_id: OrganizationId,
        address_type: AddressType,
        line1: String,
        line2: Option<String>,
        city: String,
        state_province: Option<String>,
        postal_code: Option<String>,
        country: String,
        is_primary: bool,
        is_billing_address: bool,
        is_shipping_address: bool,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create address component
        let address = Address::new(line1, line2, city.clone(), state_province, postal_code, country.clone())
            .map_err(|e| DomainError::ValidationError(e))?;
        let address_data = AddressComponentData {
            address_type,
            address,
            is_primary,
            is_billing_address,
            is_shipping_address,
        };
        
        let component = ComponentInstance::new(organization_id, address_data)?;
        let component_id = component.id;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::AddressAdded {
            organization_id,
            component_id,
            address_type,
            city,
            country,
            is_primary,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_add_certification(
        &self,
        organization_id: OrganizationId,
        certification_type: CertificationType,
        name: String,
        issuing_body: String,
        certification_number: Option<String>,
        issue_date: chrono::NaiveDate,
        expiry_date: Option<chrono::NaiveDate>,
        scope: Option<String>,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create certification component
        let cert_data = CertificationComponentData {
            certification_type,
            name: name.clone(),
            issuing_body: issuing_body.clone(),
            certification_number,
            issue_date,
            expiry_date,
            status: CertificationStatus::Active,
            scope,
        };
        
        let component = ComponentInstance::new(organization_id, cert_data)?;
        let component_id = component.id;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::CertificationAdded {
            organization_id,
            component_id,
            certification_type,
            name,
            issuing_body,
            issue_date,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_add_industry(
        &self,
        organization_id: OrganizationId,
        classification_system: ClassificationSystem,
        code: String,
        description: String,
        is_primary: bool,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create industry component
        let industry_data = IndustryComponentData {
            classification_system,
            code: code.clone(),
            description: description.clone(),
            is_primary,
        };
        
        let component = ComponentInstance::new(organization_id, industry_data)?;
        let component_id = component.id;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::IndustryClassificationAdded {
            organization_id,
            component_id,
            classification_system,
            code,
            description,
            is_primary,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_set_financial_info(
        &self,
        organization_id: OrganizationId,
        fiscal_year_end: Option<String>,
        revenue_range: Option<crate::components::data::RevenueRange>,
        employee_count_range: Option<crate::components::data::EmployeeRange>,
        credit_rating: Option<String>,
        duns_number: Option<String>,
        tax_id: Option<String>,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create financial component
        let financial_data = FinancialComponentData {
            fiscal_year_end,
            revenue_range,
            employee_count_range,
            credit_rating,
            duns_number,
            tax_id,
        };
        
        let component = ComponentInstance::new(organization_id, financial_data)?;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::FinancialInfoSet {
            organization_id,
            revenue_range,
            employee_count_range,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_add_social_profile(
        &self,
        organization_id: OrganizationId,
        platform: SocialPlatform,
        profile_url: String,
        handle: String,
        is_verified: bool,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create social media component
        let social_data = SocialMediaComponentData {
            platform,
            profile_url: profile_url.clone(),
            handle: handle.clone(),
            is_verified,
            follower_count: None,
        };
        
        let component = ComponentInstance::new(organization_id, social_data)?;
        let component_id = component.id;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::SocialProfileAdded {
            organization_id,
            component_id,
            platform,
            handle,
            profile_url,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
    
    async fn handle_add_partnership(
        &self,
        organization_id: OrganizationId,
        partner_organization_id: Option<OrganizationId>,
        partner_name: String,
        partnership_type: PartnershipType,
        start_date: chrono::NaiveDate,
        end_date: Option<chrono::NaiveDate>,
        description: Option<String>,
    ) -> DomainResult<Vec<ComponentDataEvent>> {
        // Create partnership component
        let partnership_data = PartnershipComponentData {
            partner_organization_id,
            partner_name: partner_name.clone(),
            partnership_type,
            start_date,
            end_date,
            is_active: end_date.is_none() || end_date > Some(chrono::Utc::now().date_naive()),
            description,
        };
        
        let component = ComponentInstance::new(organization_id, partnership_data)?;
        let component_id = component.id;
        
        // Store component
        self.component_store.store_component(component).await?;
        
        // Create event
        let event = ComponentDataEvent::PartnershipAdded {
            organization_id,
            component_id,
            partner_name,
            partnership_type,
            start_date,
            timestamp: Utc::now(),
        };
        
        self.event_store.append(event.clone()).await?;
        
        Ok(vec![event])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use uuid::Uuid;
    use crate::infrastructure::InMemoryEventStore;
    
    #[tokio::test]
    async fn test_add_contact_component() {
        // Setup
        let event_store = Arc::new(InMemoryEventStore::new());
        let component_store = Arc::new(InMemoryComponentStore::new());
        let org_repository = Arc::new(OrganizationRepository);
        
        let handler = ComponentCommandHandler::new(
            event_store.clone(),
            component_store.clone(),
            org_repository,
        );
        
        let org_id = Uuid::new_v4();
        
        // Execute command
        let command = ComponentCommand::AddContact {
            organization_id: org_id,
            contact_type: ContactType::Main,
            phone_number: "+1-555-1234".to_string(),
            extension: Some("123".to_string()),
            department: Some("Sales".to_string()),
            hours_of_operation: Some("9-5 EST".to_string()),
            is_primary: true,
        };
        
        // Note: This will fail because organization doesn't exist, which is expected
        let result = handler.handle(command).await;
        assert!(result.is_err());
        
        // In a real test, we would first create the organization
    }
    
    #[tokio::test]
    async fn test_add_address_component() {
        // Setup
        let event_store = Arc::new(InMemoryEventStore::new());
        let component_store = Arc::new(InMemoryComponentStore::new());
        let org_repository = Arc::new(OrganizationRepository);
        
        let handler = ComponentCommandHandler::new(
            event_store.clone(),
            component_store.clone(),
            org_repository,
        );
        
        let org_id = Uuid::new_v4();
        
        // Execute command
        let command = ComponentCommand::AddAddress {
            organization_id: org_id,
            address_type: AddressType::Headquarters,
            line1: "123 Main St".to_string(),
            line2: Some("Suite 100".to_string()),
            city: "San Francisco".to_string(),
            state_province: Some("CA".to_string()),
            postal_code: Some("94105".to_string()),
            country: "USA".to_string(),
            is_primary: true,
            is_billing_address: true,
            is_shipping_address: false,
        };
        
        // Note: This will fail because organization doesn't exist, which is expected
        let result = handler.handle(command).await;
        assert!(result.is_err());
    }
} 