//! Role level value object

use serde::{Deserialize, Serialize};

/// Role levels for organizational hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RoleLevel {
    /// C-level executives (CEO, CTO, CFO, etc.)
    Executive,
    /// Vice Presidents and Senior Vice Presidents
    VicePresident,
    /// Directors overseeing multiple teams or departments
    Director,
    /// Managers responsible for teams
    Manager,
    /// Team leads with technical leadership responsibilities
    Lead,
    /// Senior individual contributors
    Senior,
    /// Mid-level individual contributors
    Mid,
    /// Junior individual contributors
    Junior,
    /// Entry-level employees
    Entry,
    /// Interns and trainees
    Intern,
}

impl RoleLevel {
    /// Get the numeric level (lower = higher rank)
    pub fn numeric_level(&self) -> u8 {
        match self {
            Self::Executive => 1,
            Self::VicePresident => 2,
            Self::Director => 3,
            Self::Manager => 4,
            Self::Lead => 5,
            Self::Senior => 6,
            Self::Mid => 7,
            Self::Junior => 8,
            Self::Entry => 9,
            Self::Intern => 10,
        }
    }

    /// Check if this level can manage another level
    pub fn can_manage(&self, other: &RoleLevel) -> bool {
        self.numeric_level() < other.numeric_level()
    }

    /// Check if this is a management level
    pub fn is_management(&self) -> bool {
        matches!(
            self,
            Self::Executive | Self::VicePresident | Self::Director | Self::Manager | Self::Lead
        )
    }

    /// Check if this is an individual contributor level
    pub fn is_individual_contributor(&self) -> bool {
        matches!(
            self,
            Self::Senior | Self::Mid | Self::Junior | Self::Entry | Self::Intern
        )
    }

    /// Get typical reporting span for this level
    pub fn typical_reporting_span(&self) -> (u8, u8) {
        match self {
            Self::Executive => (3, 10),      // 3-10 direct reports
            Self::VicePresident => (3, 8),   // 3-8 direct reports
            Self::Director => (3, 7),         // 3-7 direct reports
            Self::Manager => (3, 10),         // 3-10 direct reports
            Self::Lead => (2, 6),             // 2-6 direct reports
            _ => (0, 0),                      // No direct reports
        }
    }
}

impl Default for RoleLevel {
    fn default() -> Self {
        Self::Mid
    }
}

impl std::fmt::Display for RoleLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Executive => write!(f, "Executive"),
            Self::VicePresident => write!(f, "Vice President"),
            Self::Director => write!(f, "Director"),
            Self::Manager => write!(f, "Manager"),
            Self::Lead => write!(f, "Lead"),
            Self::Senior => write!(f, "Senior"),
            Self::Mid => write!(f, "Mid-Level"),
            Self::Junior => write!(f, "Junior"),
            Self::Entry => write!(f, "Entry-Level"),
            Self::Intern => write!(f, "Intern"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        assert!(RoleLevel::Executive.can_manage(&RoleLevel::Director));
        assert!(RoleLevel::Manager.can_manage(&RoleLevel::Senior));
        assert!(!RoleLevel::Junior.can_manage(&RoleLevel::Senior));
    }

    #[test]
    fn test_role_categories() {
        assert!(RoleLevel::Director.is_management());
        assert!(!RoleLevel::Senior.is_management());
        assert!(RoleLevel::Junior.is_individual_contributor());
        assert!(!RoleLevel::Manager.is_individual_contributor());
    }

    #[test]
    fn test_reporting_span() {
        let (min, max) = RoleLevel::Manager.typical_reporting_span();
        assert_eq!(min, 3);
        assert_eq!(max, 10);
        
        let (min, max) = RoleLevel::Senior.typical_reporting_span();
        assert_eq!(min, 0);
        assert_eq!(max, 0);
    }
} 