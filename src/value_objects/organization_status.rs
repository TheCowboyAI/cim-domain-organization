//! Organization status value object

use serde::{Deserialize, Serialize};

/// Status of an organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganizationStatus {
    /// Actively operating
    Active,
    /// Temporarily inactive
    Inactive,
    /// In the process of being set up
    Pending,
    /// Merged with another organization
    Merged,
    /// Acquired by another organization
    Acquired,
    /// Dissolved or closed
    Dissolved,
    /// Archived/historical
    Archived,
}

impl OrganizationStatus {
    /// Check if the organization is operational
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Active | Self::Pending)
    }

    /// Check if the organization can have members
    pub fn can_have_members(&self) -> bool {
        matches!(self, Self::Active | Self::Pending | Self::Inactive)
    }

    /// Check if the organization can be modified
    pub fn can_be_modified(&self) -> bool {
        matches!(self, Self::Active | Self::Pending | Self::Inactive)
    }

    /// Get valid transitions from this status
    pub fn valid_transitions(&self) -> Vec<OrganizationStatus> {
        match self {
            Self::Pending => vec![Self::Active, Self::Dissolved],
            Self::Active => vec![Self::Inactive, Self::Merged, Self::Acquired, Self::Dissolved],
            Self::Inactive => vec![Self::Active, Self::Dissolved, Self::Archived],
            Self::Merged | Self::Acquired | Self::Dissolved => vec![Self::Archived],
            Self::Archived => vec![], // No transitions from archived
        }
    }

    /// Check if a transition to another status is valid
    pub fn can_transition_to(&self, new_status: &OrganizationStatus) -> bool {
        self.valid_transitions().contains(new_status)
    }
}

impl Default for OrganizationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for OrganizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Inactive => write!(f, "Inactive"),
            Self::Pending => write!(f, "Pending"),
            Self::Merged => write!(f, "Merged"),
            Self::Acquired => write!(f, "Acquired"),
            Self::Dissolved => write!(f, "Dissolved"),
            Self::Archived => write!(f, "Archived"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_transitions() {
        assert!(OrganizationStatus::Pending.can_transition_to(&OrganizationStatus::Active));
        assert!(OrganizationStatus::Active.can_transition_to(&OrganizationStatus::Inactive));
        assert!(!OrganizationStatus::Archived.can_transition_to(&OrganizationStatus::Active));
    }

    #[test]
    fn test_operational_status() {
        assert!(OrganizationStatus::Active.is_operational());
        assert!(!OrganizationStatus::Dissolved.is_operational());
        assert!(OrganizationStatus::Active.can_have_members());
        assert!(!OrganizationStatus::Archived.can_have_members());
    }
} 