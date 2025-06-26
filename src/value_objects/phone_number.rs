//! Phone number value object

use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated phone number
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Create a new phone number with basic validation
    pub fn new(number: String) -> Result<Self, String> {
        let cleaned = number.chars()
            .filter(|c| c.is_numeric() || *c == '+' || *c == '-' || *c == ' ' || *c == '(' || *c == ')')
            .collect::<String>();
        
        if cleaned.is_empty() {
            return Err("Phone number cannot be empty".to_string());
        }
        
        // Basic validation - at least 7 digits
        let digit_count = cleaned.chars().filter(|c| c.is_numeric()).count();
        if digit_count < 7 {
            return Err("Phone number must have at least 7 digits".to_string());
        }
        
        Ok(PhoneNumber(number))
    }
    
    /// Get the phone number as a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_phone_number() {
        let phone = PhoneNumber::new("+1 (555) 123-4567".to_string()).unwrap();
        assert_eq!(phone.as_str(), "+1 (555) 123-4567");
    }
    
    #[test]
    fn test_invalid_phone_number() {
        assert!(PhoneNumber::new("123".to_string()).is_err());
        assert!(PhoneNumber::new("".to_string()).is_err());
    }
} 