//! NATS subjects and routing for the organization domain
//!
//! Defines the NATS subjects used for organization domain commands and events.

use cim_domain::{Subject, SubjectError};

/// Base subject prefix for organization domain
pub const ORGANIZATION_DOMAIN: &str = "organization";

/// Subject patterns for organization commands
pub fn organization_command_subject(command_type: &str) -> Result<Subject, SubjectError> {
    Subject::parse(&format!("{}.commands.{}", ORGANIZATION_DOMAIN, command_type))
}

/// Subject patterns for organization events
pub fn organization_event_subject(event_type: &str) -> Result<Subject, SubjectError> {
    Subject::parse(&format!("{}.events.{}", ORGANIZATION_DOMAIN, event_type))
}

/// Subject for organization queries
pub fn organization_query_subject(query_type: &str) -> Result<Subject, SubjectError> {
    Subject::parse(&format!("{}.queries.{}", ORGANIZATION_DOMAIN, query_type))
}

/// Subject for organization snapshots/projections
pub fn organization_snapshot_subject(organization_id: &str) -> Result<Subject, SubjectError> {
    Subject::parse(&format!("{}.snapshots.{}", ORGANIZATION_DOMAIN, organization_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_subjects() {
        let subject = organization_command_subject("create").unwrap();
        assert_eq!(format!("{}", subject), "organization.commands.create");
    }

    #[test]
    fn test_event_subjects() {
        let subject = organization_event_subject("created").unwrap();
        assert_eq!(format!("{}", subject), "organization.events.created");
    }

    #[test]
    fn test_query_subjects() {
        let subject = organization_query_subject("find_by_id").unwrap();
        assert_eq!(format!("{}", subject), "organization.queries.find_by_id");
    }

    #[test]
    fn test_snapshot_subjects() {
        let subject = organization_snapshot_subject("org-123").unwrap();
        assert_eq!(format!("{}", subject), "organization.snapshots.org-123");
    }
}