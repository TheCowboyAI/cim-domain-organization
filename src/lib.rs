//! CIM Domain - Organization
//!
//! This module provides domain entities, commands, events, and aggregates
//! for organizational management within the CIM ecosystem.

pub mod entity;
pub mod events;
pub mod commands;
pub mod aggregate;
pub mod nats;

// Re-export main types
pub use entity::{Organization, Department, Team, Role, OrganizationUnit};
pub use events::{OrganizationEvent, OrganizationCreated, DepartmentCreated, TeamFormed};
pub use commands::{OrganizationCommand, CreateOrganization, CreateDepartment, CreateTeam};
pub use aggregate::OrganizationAggregate;

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

    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

/// Result type for organization operations
pub type OrganizationResult<T> = Result<T, OrganizationError>;