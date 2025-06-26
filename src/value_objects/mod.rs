//! Value objects for the Organization domain

pub mod organization_type;
pub mod organization_status;
pub mod organization_role;
pub mod role_level;
pub mod size_category;
pub mod phone_number;
pub mod address;

pub use organization_type::*;
pub use organization_status::*;
pub use organization_role::*;
pub use role_level::*;
pub use size_category::*;
pub use phone_number::*;
pub use address::*; 