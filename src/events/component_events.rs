//! Component-related events for organizations

use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};

use crate::aggregate::OrganizationId;
use crate::components::data::{
    ComponentInstanceId, ContactType, AddressType, CertificationType,
    CertificationStatus, ClassificationSystem, RevenueRange, EmployeeRange,
    SocialPlatform, PartnershipType,
};

/// Events related to organization component data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentDataEvent {
    // Contact events
    ContactAdded {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        contact_type: ContactType,
        phone_number: String,
        is_primary: bool,
        timestamp: DateTime<Utc>,
    },
    ContactUpdated {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        changes: ContactChanges,
        timestamp: DateTime<Utc>,
    },
    ContactRemoved {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        timestamp: DateTime<Utc>,
    },
    
    // Address events
    AddressAdded {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        address_type: AddressType,
        city: String,
        country: String,
        is_primary: bool,
        timestamp: DateTime<Utc>,
    },
    AddressUpdated {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        changes: AddressChanges,
        timestamp: DateTime<Utc>,
    },
    AddressRemoved {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        timestamp: DateTime<Utc>,
    },
    
    // Certification events
    CertificationAdded {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        certification_type: CertificationType,
        name: String,
        issuing_body: String,
        issue_date: NaiveDate,
        timestamp: DateTime<Utc>,
    },
    CertificationUpdated {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        status: Option<CertificationStatus>,
        expiry_date: Option<NaiveDate>,
        timestamp: DateTime<Utc>,
    },
    CertificationRemoved {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        timestamp: DateTime<Utc>,
    },
    
    // Industry events
    IndustryClassificationAdded {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        classification_system: ClassificationSystem,
        code: String,
        description: String,
        is_primary: bool,
        timestamp: DateTime<Utc>,
    },
    IndustryClassificationUpdated {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        is_primary: Option<bool>,
        timestamp: DateTime<Utc>,
    },
    IndustryClassificationRemoved {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        timestamp: DateTime<Utc>,
    },
    
    // Financial events
    FinancialInfoSet {
        organization_id: OrganizationId,
        revenue_range: Option<RevenueRange>,
        employee_count_range: Option<EmployeeRange>,
        timestamp: DateTime<Utc>,
    },
    FinancialInfoUpdated {
        organization_id: OrganizationId,
        revenue_range: Option<RevenueRange>,
        employee_count_range: Option<EmployeeRange>,
        credit_rating: Option<String>,
        timestamp: DateTime<Utc>,
    },
    
    // Social media events
    SocialProfileAdded {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        platform: SocialPlatform,
        handle: String,
        profile_url: String,
        timestamp: DateTime<Utc>,
    },
    SocialProfileUpdated {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        changes: SocialProfileChanges,
        timestamp: DateTime<Utc>,
    },
    SocialProfileRemoved {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        timestamp: DateTime<Utc>,
    },
    
    // Partnership events
    PartnershipAdded {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        partner_name: String,
        partnership_type: PartnershipType,
        start_date: NaiveDate,
        timestamp: DateTime<Utc>,
    },
    PartnershipUpdated {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        end_date: Option<NaiveDate>,
        is_active: Option<bool>,
        timestamp: DateTime<Utc>,
    },
    PartnershipRemoved {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        timestamp: DateTime<Utc>,
    },
}

/// Changes to contact information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactChanges {
    pub phone_number: Option<String>,
    pub extension: Option<String>,
    pub department: Option<String>,
    pub hours_of_operation: Option<String>,
    pub is_primary: Option<bool>,
}

/// Changes to address information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressChanges {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub is_primary: Option<bool>,
    pub is_billing_address: Option<bool>,
    pub is_shipping_address: Option<bool>,
}

/// Changes to social profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialProfileChanges {
    pub profile_url: Option<String>,
    pub handle: Option<String>,
    pub is_verified: Option<bool>,
    pub follower_count: Option<u64>,
} 