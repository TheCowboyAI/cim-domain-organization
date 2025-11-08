//! CIM Domain - Organization
//!
//! This module provides domain entities, commands, events, and aggregates
//! for organizational management within the CIM ecosystem.

pub mod entity;
pub mod events;
pub mod commands;
pub mod aggregate;
pub mod nats;
pub mod ports;
pub mod adapters;

// Re-export main types
pub use entity::{Organization, Department, Team, Role, OrganizationUnit, OrganizationType, OrganizationStatus};
pub use aggregate::{OrganizationAggregate, OrganizationMember, OrganizationRole, RoleLevel, OrganizationLocation, Permission};
pub use events::{
    OrganizationEvent, OrganizationCreated, DepartmentCreated, TeamFormed,
    MemberAdded, MemberRoleUpdated, MemberRemoved, ReportingRelationshipChanged,
    LocationAdded, PrimaryLocationChanged, LocationRemoved, OrganizationStatusChanged
};
pub use commands::{
    OrganizationCommand, CreateOrganization, CreateDepartment, CreateTeam,
    AddMember, UpdateMemberRole, RemoveMember, ChangeReportingRelationship,
    AddLocation, ChangePrimaryLocation, RemoveLocation,
    AddChildOrganization, RemoveChildOrganization, ChangeOrganizationStatus,
    DissolveOrganization, MergeOrganizations, UpdateOrganization
};
pub use cim_domain::{EntityId, MessageIdentity};

use cim_domain::DomainError;
use thiserror::Error;

/// Domain-specific errors for organization operations
#[derive(Debug, Error)]
pub enum OrganizationError {
    #[error("Organization not found: {0}")]
    OrganizationNotFound(uuid::Uuid),

    #[error("Department not found: {0}")]
    DepartmentNotFound(uuid::Uuid),

    #[error("Team not found: {0}")]
    TeamNotFound(uuid::Uuid),

    #[error("Invalid organizational structure: {0}")]
    InvalidStructure(String),

    #[error("Duplicate entity: {0}")]
    DuplicateEntity(String),

    #[error("Circular reference: {0}")]
    CircularReference(String),

    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

/// Result type for organization operations
pub type OrganizationResult<T> = Result<T, OrganizationError>;

/// Organization size categories based on employee count
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeCategory {
    /// 1-10 employees
    Startup,
    /// 11-50 employees
    Small,
    /// 51-250 employees
    Medium,
    /// 251-1000 employees
    Large,
    /// 1001-5000 employees
    Enterprise,
    /// 5000+ employees
    MegaCorp,
}

impl SizeCategory {
    /// Determine size category from employee count
    pub fn from_employee_count(count: usize) -> Self {
        match count {
            0..=10 => SizeCategory::Startup,
            11..=50 => SizeCategory::Small,
            51..=250 => SizeCategory::Medium,
            251..=1000 => SizeCategory::Large,
            1001..=5000 => SizeCategory::Enterprise,
            _ => SizeCategory::MegaCorp,
        }
    }

    /// Get typical management layers for this size category
    pub fn typical_management_layers(&self) -> u8 {
        match self {
            SizeCategory::Startup => 2,
            SizeCategory::Small => 3,
            SizeCategory::Medium => 4,
            SizeCategory::Large => 5,
            SizeCategory::Enterprise => 6,
            SizeCategory::MegaCorp => 7,
        }
    }

    /// Get employee range for this size category
    pub fn employee_range(&self) -> (usize, Option<usize>) {
        match self {
            SizeCategory::Startup => (1, Some(10)),
            SizeCategory::Small => (11, Some(50)),
            SizeCategory::Medium => (51, Some(250)),
            SizeCategory::Large => (251, Some(1000)),
            SizeCategory::Enterprise => (1001, Some(5000)),
            SizeCategory::MegaCorp => (5001, None),
        }
    }

    /// Get typical budget range for this size category (in millions USD)
    pub fn typical_budget_range(&self) -> (f64, f64) {
        match self {
            SizeCategory::Startup => (0.1, 5.0),
            SizeCategory::Small => (5.0, 25.0),
            SizeCategory::Medium => (25.0, 100.0),
            SizeCategory::Large => (100.0, 500.0),
            SizeCategory::Enterprise => (500.0, 2000.0),
            SizeCategory::MegaCorp => (2000.0, 50000.0),
        }
    }
}