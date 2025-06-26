//! Address value object

use serde::{Deserialize, Serialize};
use std::fmt;

/// A physical or mailing address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
}

impl Address {
    /// Create a new address with validation
    pub fn new(
        line1: String,
        line2: Option<String>,
        city: String,
        state_province: Option<String>,
        postal_code: Option<String>,
        country: String,
    ) -> Result<Self, String> {
        if line1.trim().is_empty() {
            return Err("Address line 1 cannot be empty".to_string());
        }
        
        if city.trim().is_empty() {
            return Err("City cannot be empty".to_string());
        }
        
        if country.trim().is_empty() {
            return Err("Country cannot be empty".to_string());
        }
        
        Ok(Address {
            line1,
            line2,
            city,
            state_province,
            postal_code,
            country,
        })
    }
    
    /// Format address as a single string
    pub fn format_single_line(&self) -> String {
        let mut parts = vec![self.line1.clone()];
        
        if let Some(line2) = &self.line2 {
            parts.push(line2.clone());
        }
        
        parts.push(self.city.clone());
        
        if let Some(state) = &self.state_province {
            parts.push(state.clone());
        }
        
        if let Some(postal) = &self.postal_code {
            parts.push(postal.clone());
        }
        
        parts.push(self.country.clone());
        
        parts.join(", ")
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_single_line())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_address() {
        let addr = Address::new(
            "123 Main St".to_string(),
            Some("Suite 100".to_string()),
            "San Francisco".to_string(),
            Some("CA".to_string()),
            Some("94105".to_string()),
            "USA".to_string(),
        ).unwrap();
        
        assert_eq!(addr.line1, "123 Main St");
        assert_eq!(addr.city, "San Francisco");
    }
    
    #[test]
    fn test_invalid_address() {
        assert!(Address::new(
            "".to_string(),
            None,
            "City".to_string(),
            None,
            None,
            "Country".to_string(),
        ).is_err());
    }
} 