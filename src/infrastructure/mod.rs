//! Infrastructure layer for the Organization domain
//!
//! This module contains all infrastructure concerns:
//! - NATS integration
//! - Event store
//! - Repository pattern
//! - Snapshot storage

pub mod nats_integration;
pub mod persistence;
