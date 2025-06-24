//! # Organization Domain
//!
//! The Organization domain manages organizational structures, hierarchies, and relationships
//! within the CIM system. It provides:
//!
//! - Hierarchical organizational units (companies, divisions, departments, teams)
//! - Role-based member assignments with reporting structures
//! - Multiple location associations
//! - Event-driven state management
//!
//! ## Architecture
//!
//! This domain follows event-sourcing and CQRS patterns:
//! - Commands express intent to change organizational state
//! - Events record what actually happened
//! - Projections provide optimized read models
//! - Aggregates enforce business rules and invariants

pub mod aggregate;
pub mod commands;
pub mod events;
pub mod handlers;
pub mod projections;
pub mod queries;
pub mod value_objects;

// Re-export main types
pub use aggregate::*;
pub use commands::*;
pub use events::*;
pub use handlers::*;
pub use projections::*;
pub use queries::*;
pub use value_objects::*; 