//! Size category value object

use serde::{Deserialize, Serialize};

/// Size classification for organizations based on employee count
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Get the size category based on employee count
    pub fn from_employee_count(count: usize) -> Self {
        match count {
            0..=10 => Self::Startup,
            11..=50 => Self::Small,
            51..=250 => Self::Medium,
            251..=1000 => Self::Large,
            1001..=5000 => Self::Enterprise,
            _ => Self::MegaCorp,
        }
    }

    /// Get the employee count range for this category
    pub fn employee_range(&self) -> (usize, Option<usize>) {
        match self {
            Self::Startup => (1, Some(10)),
            Self::Small => (11, Some(50)),
            Self::Medium => (51, Some(250)),
            Self::Large => (251, Some(1000)),
            Self::Enterprise => (1001, Some(5000)),
            Self::MegaCorp => (5001, None),
        }
    }

    /// Get typical budget range (in millions USD) for this size
    pub fn typical_budget_range(&self) -> (f64, Option<f64>) {
        match self {
            Self::Startup => (0.1, Some(1.0)),
            Self::Small => (1.0, Some(10.0)),
            Self::Medium => (10.0, Some(50.0)),
            Self::Large => (50.0, Some(500.0)),
            Self::Enterprise => (500.0, Some(5000.0)),
            Self::MegaCorp => (5000.0, None),
        }
    }

    /// Get typical number of departments for this size
    pub fn typical_department_count(&self) -> (usize, usize) {
        match self {
            Self::Startup => (1, 3),
            Self::Small => (3, 8),
            Self::Medium => (8, 20),
            Self::Large => (20, 50),
            Self::Enterprise => (50, 200),
            Self::MegaCorp => (200, 1000),
        }
    }

    /// Get typical management layers for this size
    pub fn typical_management_layers(&self) -> usize {
        match self {
            Self::Startup => 2,    // CEO -> Everyone
            Self::Small => 3,      // CEO -> Managers -> ICs
            Self::Medium => 4,     // CEO -> Directors -> Managers -> ICs
            Self::Large => 5,      // CEO -> VPs -> Directors -> Managers -> ICs
            Self::Enterprise => 6, // CEO -> EVPs -> VPs -> Directors -> Managers -> ICs
            Self::MegaCorp => 7,   // Additional layer
        }
    }
}

impl Default for SizeCategory {
    fn default() -> Self {
        Self::Small
    }
}

impl std::fmt::Display for SizeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Startup => write!(f, "Startup (1-10 employees)"),
            Self::Small => write!(f, "Small (11-50 employees)"),
            Self::Medium => write!(f, "Medium (51-250 employees)"),
            Self::Large => write!(f, "Large (251-1000 employees)"),
            Self::Enterprise => write!(f, "Enterprise (1001-5000 employees)"),
            Self::MegaCorp => write!(f, "MegaCorp (5000+ employees)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_category_from_count() {
        assert_eq!(SizeCategory::from_employee_count(5), SizeCategory::Startup);
        assert_eq!(SizeCategory::from_employee_count(25), SizeCategory::Small);
        assert_eq!(SizeCategory::from_employee_count(100), SizeCategory::Medium);
        assert_eq!(SizeCategory::from_employee_count(500), SizeCategory::Large);
        assert_eq!(SizeCategory::from_employee_count(2500), SizeCategory::Enterprise);
        assert_eq!(SizeCategory::from_employee_count(10000), SizeCategory::MegaCorp);
    }

    #[test]
    fn test_employee_ranges() {
        let (min, max) = SizeCategory::Startup.employee_range();
        assert_eq!(min, 1);
        assert_eq!(max, Some(10));

        let (min, max) = SizeCategory::MegaCorp.employee_range();
        assert_eq!(min, 5001);
        assert_eq!(max, None);
    }

    #[test]
    fn test_management_layers() {
        assert_eq!(SizeCategory::Startup.typical_management_layers(), 2);
        assert_eq!(SizeCategory::Enterprise.typical_management_layers(), 6);
    }
} 