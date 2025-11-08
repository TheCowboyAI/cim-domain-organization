//! Organization domain entities
//!
//! Core entities for organizational management following DDD principles

use chrono::{DateTime, Utc};
use cim_domain::{DomainEntity, EntityId};
use serde::{Deserialize, Serialize};

/// Organization entity - represents a company, business unit, or institution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Organization {
    pub id: EntityId<Organization>,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub parent_id: Option<EntityId<Organization>>,
    pub organization_type: OrganizationType,
    pub status: OrganizationStatus,
    pub founded_date: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEntity for Organization {
    type IdType = Organization;

    fn id(&self) -> EntityId<Self::IdType> {
        self.id.clone()
    }
}

/// Organization types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrganizationType {
    Corporation,
    NonProfit,
    Government,
    Partnership,
    SoleProprietorship,
    Cooperative,
    LLC,
    Other(String),
}

/// Organization status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrganizationStatus {
    Pending,
    Active,
    Inactive,
    Suspended,
    Dissolved,
    Merged,
}

/// Department entity - a division within an organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Department {
    pub id: EntityId<Department>,
    pub organization_id: EntityId<Organization>,
    pub parent_department_id: Option<EntityId<Department>>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub head_role_id: Option<EntityId<Role>>,
    pub status: DepartmentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEntity for Department {
    type IdType = Department;

    fn id(&self) -> EntityId<Self::IdType> {
        self.id.clone()
    }
}

/// Department status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DepartmentStatus {
    Active,
    Inactive,
    Restructuring,
    Dissolved,
}

/// Team entity - a working group within a department or organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Team {
    pub id: EntityId<Team>,
    pub organization_id: EntityId<Organization>,
    pub department_id: Option<EntityId<Department>>,
    pub name: String,
    pub description: Option<String>,
    pub team_type: TeamType,
    pub lead_role_id: Option<EntityId<Role>>,
    pub max_members: Option<usize>,
    pub status: TeamStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEntity for Team {
    type IdType = Team;

    fn id(&self) -> EntityId<Self::IdType> {
        self.id.clone()
    }
}

/// Team types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TeamType {
    Permanent,
    Project,
    CrossFunctional,
    Virtual,
    SelfManaged,
    TaskForce,
}

/// Team status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TeamStatus {
    Forming,
    Active,
    OnHold,
    Disbanding,
    Disbanded,
}

/// Role entity - a position or responsibility within an organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Role {
    pub id: EntityId<Role>,
    pub organization_id: EntityId<Organization>,
    pub department_id: Option<EntityId<Department>>,
    pub team_id: Option<EntityId<Team>>,
    pub title: String,
    pub code: String,
    pub description: Option<String>,
    pub role_type: RoleType,
    pub level: Option<u8>,
    pub reports_to: Option<EntityId<Role>>,
    pub permissions: Vec<String>,
    pub responsibilities: Vec<String>,
    pub status: RoleStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEntity for Role {
    type IdType = Role;

    fn id(&self) -> EntityId<Self::IdType> {
        self.id.clone()
    }
}

/// Role types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RoleType {
    Executive,
    Management,
    Technical,
    Administrative,
    Operational,
    Support,
    Contractor,
    Intern,
}

/// Role status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RoleStatus {
    Active,
    Vacant,
    Deprecated,
    Frozen,
}

/// Facility entity - an organizational place (office, warehouse, etc.)
/// NOTE: This does NOT contain location address data - that's in the Location domain
/// The facility is linked to a Location via a separate relationship/association
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Facility {
    pub id: EntityId<Facility>,
    pub organization_id: EntityId<Organization>,
    pub name: String,
    pub code: String,
    pub facility_type: FacilityType,
    pub description: Option<String>,
    pub capacity: Option<u32>,
    pub status: FacilityStatus,
    pub parent_facility_id: Option<EntityId<Facility>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEntity for Facility {
    type IdType = Facility;

    fn id(&self) -> EntityId<Self::IdType> {
        self.id.clone()
    }
}

/// Facility types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FacilityType {
    Headquarters,
    Office,
    Warehouse,
    Factory,
    RetailStore,
    DataCenter,
    Laboratory,
    ServiceCenter,
    Other(String),
}

/// Facility status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FacilityStatus {
    Active,
    UnderConstruction,
    Renovating,
    Inactive,
    Closed,
}

/// Organizational Unit - generic container for organizational structures
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrganizationUnit {
    pub id: EntityId<OrganizationUnit>,
    pub organization_id: EntityId<Organization>,
    pub parent_id: Option<EntityId<OrganizationUnit>>,
    pub unit_type: OrganizationUnitType,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEntity for OrganizationUnit {
    type IdType = OrganizationUnit;

    fn id(&self) -> EntityId<Self::IdType> {
        self.id.clone()
    }
}

/// Organization unit types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrganizationUnitType {
    Division,
    Branch,
    Office,
    Region,
    District,
    CostCenter,
    ProfitCenter,
    BusinessUnit,
    Other(String),
}

// Builder patterns for easier entity creation

impl Organization {
    pub fn builder(name: String) -> OrganizationBuilder {
        OrganizationBuilder::new(name)
    }
}

pub struct OrganizationBuilder {
    name: String,
    display_name: Option<String>,
    description: Option<String>,
    parent_id: Option<EntityId<Organization>>,
    organization_type: OrganizationType,
    founded_date: Option<DateTime<Utc>>,
    metadata: serde_json::Value,
}

impl OrganizationBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            description: None,
            parent_id: None,
            organization_type: OrganizationType::Corporation,
            founded_date: None,
            metadata: serde_json::Value::Object(serde_json::Map::new()),
        }
    }

    pub fn with_display_name(mut self, display_name: String) -> Self {
        self.display_name = Some(display_name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_parent(mut self, parent_id: EntityId<Organization>) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn with_type(mut self, org_type: OrganizationType) -> Self {
        self.organization_type = org_type;
        self
    }

    pub fn with_founded_date(mut self, date: DateTime<Utc>) -> Self {
        self.founded_date = Some(date);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn build(self) -> Organization {
        let now = Utc::now();
        Organization {
            id: EntityId::new(),
            name: self.name.clone(),
            display_name: self.display_name.unwrap_or_else(|| self.name),
            description: self.description,
            parent_id: self.parent_id,
            organization_type: self.organization_type,
            status: OrganizationStatus::Active,
            founded_date: self.founded_date,
            metadata: self.metadata,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Department {
    pub fn new(
        organization_id: EntityId<Organization>,
        name: String,
        code: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EntityId::new(),
            organization_id,
            parent_department_id: None,
            name,
            code,
            description: None,
            head_role_id: None,
            status: DepartmentStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Team {
    pub fn new(
        organization_id: EntityId<Organization>,
        name: String,
        team_type: TeamType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EntityId::new(),
            organization_id,
            department_id: None,
            name,
            description: None,
            team_type,
            lead_role_id: None,
            max_members: None,
            status: TeamStatus::Forming,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Role {
    pub fn new(
        organization_id: EntityId<Organization>,
        title: String,
        code: String,
        role_type: RoleType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EntityId::new(),
            organization_id,
            department_id: None,
            team_id: None,
            title,
            code,
            description: None,
            role_type,
            level: None,
            reports_to: None,
            permissions: Vec::new(),
            responsibilities: Vec::new(),
            status: RoleStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Facility {
    pub fn new(
        organization_id: EntityId<Organization>,
        name: String,
        code: String,
        facility_type: FacilityType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EntityId::new(),
            organization_id,
            name,
            code,
            facility_type,
            description: None,
            capacity: None,
            status: FacilityStatus::Active,
            parent_facility_id: None,
            created_at: now,
            updated_at: now,
        }
    }
}