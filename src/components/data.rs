//! Component data structures for organizations

use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;
use cim_domain::DomainResult;

use crate::aggregate::OrganizationId;
use crate::value_objects::{PhoneNumber, Address};
use super::{ComponentMetadata, ComponentType, OrganizationComponent};

/// Unique identifier for component instances
pub type ComponentInstanceId = Uuid;

/// A component instance with its data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentInstance<T> {
    pub id: ComponentInstanceId,
    pub organization_id: OrganizationId,
    pub data: T,
    pub metadata: ComponentMetadata,
}

impl<T> ComponentInstance<T> {
    /// Create a new component instance
    pub fn new(organization_id: OrganizationId, data: T) -> DomainResult<Self> {
        Ok(Self {
            id: Uuid::new_v4(),
            organization_id,
            data,
            metadata: ComponentMetadata {
                attached_at: Utc::now(),
                updated_at: Utc::now(),
                source: "system".to_string(),
                version: 1,
            },
        })
    }
}

// ===== Contact Components =====

/// Organization contact information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactComponentData {
    pub contact_type: ContactType,
    pub phone: PhoneNumber,
    pub extension: Option<String>,
    pub department: Option<String>,
    pub hours_of_operation: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactType {
    Main,
    Sales,
    Support,
    Billing,
    Emergency,
    Other,
}

impl OrganizationComponent for ContactComponentData {
    fn component_type() -> ComponentType {
        ComponentType::Contact
    }
}

// ===== Address Components =====

/// Organization address
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressComponentData {
    pub address_type: AddressType,
    pub address: Address,
    pub is_primary: bool,
    pub is_billing_address: bool,
    pub is_shipping_address: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddressType {
    Headquarters,
    Branch,
    Warehouse,
    Manufacturing,
    Mailing,
    Registered,
    Other,
}

impl OrganizationComponent for AddressComponentData {
    fn component_type() -> ComponentType {
        ComponentType::Address
    }
}

// ===== Certification Components =====

/// Organization certifications and accreditations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificationComponentData {
    pub certification_type: CertificationType,
    pub name: String,
    pub issuing_body: String,
    pub certification_number: Option<String>,
    pub issue_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub status: CertificationStatus,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationType {
    ISO9001,
    ISO14001,
    ISO27001,
    SOC2,
    PciDss,
    License,
    Accreditation,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationStatus {
    Active,
    Expired,
    Suspended,
    Revoked,
    Pending,
}

impl OrganizationComponent for CertificationComponentData {
    fn component_type() -> ComponentType {
        ComponentType::Certification
    }
}

// ===== Industry Components =====

/// Industry classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustryComponentData {
    pub classification_system: ClassificationSystem,
    pub code: String,
    pub description: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassificationSystem {
    NAICS,  // North American Industry Classification System
    SIC,    // Standard Industrial Classification
    ISIC,   // International Standard Industrial Classification
    NACE,   // European Classification
    Other,
}

impl OrganizationComponent for IndustryComponentData {
    fn component_type() -> ComponentType {
        ComponentType::Industry
    }
}

// ===== Financial Components =====

/// Financial information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialComponentData {
    pub fiscal_year_end: Option<String>, // e.g., "12-31"
    pub revenue_range: Option<RevenueRange>,
    pub employee_count_range: Option<EmployeeRange>,
    pub credit_rating: Option<String>,
    pub duns_number: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueRange {
    Under1M,
    From1MTo10M,
    From10MTo50M,
    From50MTo100M,
    From100MTo500M,
    From500MTo1B,
    Over1B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmployeeRange {
    Under10,
    From10To50,
    From50To100,
    From100To500,
    From500To1000,
    From1000To5000,
    Over5000,
}

impl OrganizationComponent for FinancialComponentData {
    fn component_type() -> ComponentType {
        ComponentType::Financial
    }
}

// ===== Social Media Components =====

/// Organization social media profiles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialMediaComponentData {
    pub platform: SocialPlatform,
    pub profile_url: String,
    pub handle: String,
    pub is_verified: bool,
    pub follower_count: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialPlatform {
    LinkedIn,
    Twitter,
    Facebook,
    Instagram,
    YouTube,
    GitHub,
    Other,
}

impl OrganizationComponent for SocialMediaComponentData {
    fn component_type() -> ComponentType {
        ComponentType::SocialMedia
    }
}

// ===== Partnership Components =====

/// Partnerships and affiliations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartnershipComponentData {
    pub partner_organization_id: Option<OrganizationId>,
    pub partner_name: String,
    pub partnership_type: PartnershipType,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartnershipType {
    Strategic,
    Technology,
    Channel,
    Supplier,
    Customer,
    Affiliate,
    Other,
}

impl OrganizationComponent for PartnershipComponentData {
    fn component_type() -> ComponentType {
        ComponentType::Partnership
    }
} 