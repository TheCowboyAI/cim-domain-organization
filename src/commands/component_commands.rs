//! Component management commands for organizations

use crate::aggregate::OrganizationId;
use crate::components::data::{ComponentInstanceId};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Commands for managing organization components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentCommand {
    // Contact component commands
    AddContact {
        organization_id: OrganizationId,
        contact_type: crate::components::data::ContactType,
        phone_number: String,
        extension: Option<String>,
        department: Option<String>,
        hours_of_operation: Option<String>,
        is_primary: bool,
    },
    UpdateContact {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        phone_number: Option<String>,
        extension: Option<String>,
        department: Option<String>,
        hours_of_operation: Option<String>,
        is_primary: Option<bool>,
    },
    RemoveContact {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    },
    
    // Address component commands
    AddAddress {
        organization_id: OrganizationId,
        address_type: crate::components::data::AddressType,
        line1: String,
        line2: Option<String>,
        city: String,
        state_province: Option<String>,
        postal_code: Option<String>,
        country: String,
        is_primary: bool,
        is_billing_address: bool,
        is_shipping_address: bool,
    },
    UpdateAddress {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        line1: Option<String>,
        line2: Option<String>,
        city: Option<String>,
        state_province: Option<String>,
        postal_code: Option<String>,
        country: Option<String>,
        is_primary: Option<bool>,
        is_billing_address: Option<bool>,
        is_shipping_address: Option<bool>,
    },
    RemoveAddress {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    },
    
    // Certification component commands
    AddCertification {
        organization_id: OrganizationId,
        certification_type: crate::components::data::CertificationType,
        name: String,
        issuing_body: String,
        certification_number: Option<String>,
        issue_date: NaiveDate,
        expiry_date: Option<NaiveDate>,
        scope: Option<String>,
    },
    UpdateCertification {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        status: Option<crate::components::data::CertificationStatus>,
        expiry_date: Option<NaiveDate>,
        scope: Option<String>,
    },
    RemoveCertification {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    },
    
    // Industry component commands
    AddIndustry {
        organization_id: OrganizationId,
        classification_system: crate::components::data::ClassificationSystem,
        code: String,
        description: String,
        is_primary: bool,
    },
    UpdateIndustry {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        is_primary: Option<bool>,
    },
    RemoveIndustry {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    },
    
    // Financial component commands
    SetFinancialInfo {
        organization_id: OrganizationId,
        fiscal_year_end: Option<String>,
        revenue_range: Option<crate::components::data::RevenueRange>,
        employee_count_range: Option<crate::components::data::EmployeeRange>,
        credit_rating: Option<String>,
        duns_number: Option<String>,
        tax_id: Option<String>,
    },
    UpdateFinancialInfo {
        organization_id: OrganizationId,
        revenue_range: Option<crate::components::data::RevenueRange>,
        employee_count_range: Option<crate::components::data::EmployeeRange>,
        credit_rating: Option<String>,
    },
    
    // Social media component commands
    AddSocialProfile {
        organization_id: OrganizationId,
        platform: crate::components::data::SocialPlatform,
        profile_url: String,
        handle: String,
        is_verified: bool,
    },
    UpdateSocialProfile {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        profile_url: Option<String>,
        handle: Option<String>,
        is_verified: Option<bool>,
        follower_count: Option<u64>,
    },
    RemoveSocialProfile {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    },
    
    // Partnership component commands
    AddPartnership {
        organization_id: OrganizationId,
        partner_organization_id: Option<OrganizationId>,
        partner_name: String,
        partnership_type: crate::components::data::PartnershipType,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        description: Option<String>,
    },
    UpdatePartnership {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
        end_date: Option<NaiveDate>,
        is_active: Option<bool>,
        description: Option<String>,
    },
    RemovePartnership {
        organization_id: OrganizationId,
        component_id: ComponentInstanceId,
    },
} 