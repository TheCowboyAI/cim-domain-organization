//! Organization type value object

use serde::{Deserialize, Serialize};

/// Types of organizations supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganizationType {
    /// Top-level company or corporation
    Company,
    /// Major division within a company
    Division,
    /// Department within a division
    Department,
    /// Team within a department
    Team,
    /// Project-based organization
    Project,
    /// External partner organization
    Partner,
    /// Customer organization
    Customer,
    /// Vendor/supplier organization
    Vendor,
    /// Government organization
    Government,
    /// Non-profit organization
    NonProfit,
    /// Educational institution
    Educational,
    /// Healthcare organization
    Healthcare,
    /// Custom organization type
    Custom,
}

impl OrganizationType {
    /// Check if this is an internal organization type
    pub fn is_internal(&self) -> bool {
        matches!(
            self,
            Self::Company | Self::Division | Self::Department | Self::Team | Self::Project
        )
    }

    /// Check if this is an external organization type
    pub fn is_external(&self) -> bool {
        matches!(
            self,
            Self::Partner | Self::Customer | Self::Vendor | Self::Government | 
            Self::NonProfit | Self::Educational | Self::Healthcare
        )
    }

    /// Get the hierarchical level of this organization type
    /// Lower numbers indicate higher levels in the hierarchy
    pub fn hierarchical_level(&self) -> u8 {
        match self {
            Self::Company => 1,
            Self::Division => 2,
            Self::Department => 3,
            Self::Team => 4,
            Self::Project => 4, // Same level as team
            _ => 0, // External organizations don't have hierarchy
        }
    }

    /// Check if this type can be a parent of another type
    pub fn can_parent(&self, child: &OrganizationType) -> bool {
        if !self.is_internal() || !child.is_internal() {
            return false;
        }
        
        self.hierarchical_level() < child.hierarchical_level()
    }
}

impl Default for OrganizationType {
    fn default() -> Self {
        Self::Company
    }
}

impl std::fmt::Display for OrganizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Company => write!(f, "Company"),
            Self::Division => write!(f, "Division"),
            Self::Department => write!(f, "Department"),
            Self::Team => write!(f, "Team"),
            Self::Project => write!(f, "Project"),
            Self::Partner => write!(f, "Partner"),
            Self::Customer => write!(f, "Customer"),
            Self::Vendor => write!(f, "Vendor"),
            Self::Government => write!(f, "Government"),
            Self::NonProfit => write!(f, "Non-Profit"),
            Self::Educational => write!(f, "Educational"),
            Self::Healthcare => write!(f, "Healthcare"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_type_hierarchy() {
        assert!(OrganizationType::Company.can_parent(&OrganizationType::Division));
        assert!(OrganizationType::Division.can_parent(&OrganizationType::Department));
        assert!(OrganizationType::Department.can_parent(&OrganizationType::Team));
        
        assert!(!OrganizationType::Team.can_parent(&OrganizationType::Department));
        assert!(!OrganizationType::Company.can_parent(&OrganizationType::Company));
    }

    #[test]
    fn test_internal_external() {
        assert!(OrganizationType::Company.is_internal());
        assert!(OrganizationType::Team.is_internal());
        assert!(OrganizationType::Partner.is_external());
        assert!(OrganizationType::Customer.is_external());
    }
} 