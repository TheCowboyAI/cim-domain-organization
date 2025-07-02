//! Error types for the Organization domain

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrganizationError {
    #[error("Organization not found: {0}")]
    NotFound(String),
    
    #[error("Invalid organization data: {0}")]
    InvalidData(String),
    
    #[error("Cross-domain integration error: {0}")]
    IntegrationError(String),
    
    #[error("Person not found: {0}")]
    PersonNotFound(String),
    
    #[error("Location not found: {0}")]
    LocationNotFound(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    
    #[error("NATS error: {0}")]
    NatsError(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl From<serde_json::Error> for OrganizationError {
    fn from(err: serde_json::Error) -> Self {
        OrganizationError::SerializationError(err.to_string())
    }
} 