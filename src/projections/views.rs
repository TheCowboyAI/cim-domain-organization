//! View models for organization projections

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationView {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub size: usize,
    pub headquarters_location: Option<Uuid>,
    pub founded_date: Option<chrono::NaiveDate>,
    pub member_count: usize,
    pub average_tenure_days: Option<f64>,
    pub primary_location_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberView {
    pub person_id: Uuid,
    pub organization_id: Uuid,
    pub person_name: String,
    pub role: String,
    pub joined_date: chrono::DateTime<chrono::Utc>,
    pub tenure_days: i64,
} 