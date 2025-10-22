/*
 * Copyright 2025 Cowboy AI, LLC.
 * SPDX-License-Identifier: MIT
 *
 * CIM Organization Domain - NATS Subject Algebra
 * 
 * Comprehensive subject patterns for organization management operations.
 * Implements the Organization Subject Algebra: 𝒪 = (ℐ, ℰ, 𝒮, 𝒞, ⊕, ⊗, →)
 * 
 * This module provides:
 * - Organization lifecycle management (creation, updates, mergers, dissolution)
 * - Hierarchical structure management (departments, teams, roles)
 * - Employee and contractor management within organizational context
 * - Policy and governance framework
 * - Resource allocation and management
 * - Organizational analytics and reporting
 * - Compliance and audit operations
 * - Cross-organizational collaboration patterns
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

/// Core organization domain subject patterns following CIM Subject Algebra.
/// 
/// Subject Structure: `events.organization.{aggregate}.{scope}.{operation}.{entity_id}`
/// 
/// Where:
/// - aggregate: Organization, Department, Team, Role, Policy, Resource
/// - scope: Global, Organization, Department, Team, Individual
/// - operation: CRUD + domain-specific operations
/// - entity_id: Specific entity identifier (optional for queries)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrganizationSubject {
    /// Optional namespace for multi-tenant deployments
    pub namespace: Option<String>,
    
    /// Root subject pattern
    pub root: OrganizationSubjectRoot,
    
    /// Domain identifier ("organization")
    pub domain: String,
    
    /// Organization aggregate type
    pub aggregate: OrganizationAggregate,
    
    /// Scoping mechanism for operations
    pub scope: OrganizationScope,
    
    /// Optional operation specification
    pub operation: Option<String>,
    
    /// Optional entity identifier
    pub entity_id: Option<String>,
    
    /// Additional context parameters
    pub context: HashMap<String, String>,
}

/// Root subject patterns for different message types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganizationSubjectRoot {
    /// Event notifications (published after operations complete)
    Events,
    
    /// Command requests (operations to be performed)
    Commands,
    
    /// Query requests (data retrieval operations)
    Queries,
    
    /// Workflow orchestration messages
    Workflows,
    
    /// System-level operations and maintenance
    System,
    
    /// Analytics and reporting operations
    Analytics,
    
    /// Compliance and audit operations
    Compliance,
    
    /// Integration with external systems
    Integration,
}

/// Organization domain aggregates representing core business entities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganizationAggregate {
    /// Core organization entity (company, business unit)
    Organization,
    
    /// Department within organization
    Department,
    
    /// Team within department or organization
    Team,
    
    /// Organizational roles and positions
    Role,
    
    /// Organizational policies and governance
    Policy,
    
    /// Resources (financial, physical, intellectual)
    Resource,
    
    /// Organizational structure and hierarchy
    Structure,
    
    /// Organizational culture and values
    Culture,
    
    /// Strategic planning and objectives
    Strategy,
    
    /// Performance metrics and KPIs
    Performance,
    
    /// Organizational communication
    Communication,
    
    /// Change management
    Change,
    
    /// Risk management
    Risk,
    
    /// Vendor and partner management
    Vendor,
    
    /// Location and facilities management
    Location,
}

/// Scoping mechanisms for organizational operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrganizationScope {
    /// Global operations across all organizations
    Global,
    
    /// Organization-specific operations
    Organization(Uuid),
    
    /// Department-specific operations
    Department(Uuid),
    
    /// Team-specific operations
    Team(Uuid),
    
    /// Role-specific operations
    Role(Uuid),
    
    /// Location-specific operations
    Location(String),
    
    /// Region-specific operations (geographic)
    Region(String),
    
    /// Division-specific operations (business division)
    Division(Uuid),
    
    /// Project-specific operations
    Project(Uuid),
    
    /// Cost center specific operations
    CostCenter(String),
    
    /// Vendor/partner specific operations
    Vendor(Uuid),
}

impl OrganizationSubject {
    /// Creates a new organization subject with required components
    pub fn new(
        root: OrganizationSubjectRoot,
        aggregate: OrganizationAggregate,
        scope: OrganizationScope,
    ) -> Self {
        Self {
            namespace: None,
            root,
            domain: "organization".to_string(),
            aggregate,
            scope,
            operation: None,
            entity_id: None,
            context: HashMap::new(),
        }
    }
    
    /// Builder pattern for adding optional components
    pub fn with_namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }
    
    pub fn with_operation(mut self, operation: String) -> Self {
        self.operation = Some(operation);
        self
    }
    
    pub fn with_entity_id(mut self, entity_id: String) -> Self {
        self.entity_id = Some(entity_id);
        self
    }
    
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.insert(key, value);
        self
    }
    
    /// Generates the NATS subject string for this subject pattern
    pub fn to_subject_string(&self) -> String {
        let mut parts = Vec::new();
        
        // Add namespace if present
        if let Some(namespace) = &self.namespace {
            parts.push(namespace.clone());
        }
        
        // Add root pattern
        parts.push(match &self.root {
            OrganizationSubjectRoot::Events => "events".to_string(),
            OrganizationSubjectRoot::Commands => "commands".to_string(),
            OrganizationSubjectRoot::Queries => "queries".to_string(),
            OrganizationSubjectRoot::Workflows => "workflows".to_string(),
            OrganizationSubjectRoot::System => "system".to_string(),
            OrganizationSubjectRoot::Analytics => "analytics".to_string(),
            OrganizationSubjectRoot::Compliance => "compliance".to_string(),
            OrganizationSubjectRoot::Integration => "integration".to_string(),
        });
        
        // Add domain
        parts.push(self.domain.clone());
        
        // Add aggregate
        parts.push(match &self.aggregate {
            OrganizationAggregate::Organization => "organization".to_string(),
            OrganizationAggregate::Department => "department".to_string(),
            OrganizationAggregate::Team => "team".to_string(),
            OrganizationAggregate::Role => "role".to_string(),
            OrganizationAggregate::Policy => "policy".to_string(),
            OrganizationAggregate::Resource => "resource".to_string(),
            OrganizationAggregate::Structure => "structure".to_string(),
            OrganizationAggregate::Culture => "culture".to_string(),
            OrganizationAggregate::Strategy => "strategy".to_string(),
            OrganizationAggregate::Performance => "performance".to_string(),
            OrganizationAggregate::Communication => "communication".to_string(),
            OrganizationAggregate::Change => "change".to_string(),
            OrganizationAggregate::Risk => "risk".to_string(),
            OrganizationAggregate::Vendor => "vendor".to_string(),
            OrganizationAggregate::Location => "location".to_string(),
        });
        
        // Add scope
        parts.push(match &self.scope {
            OrganizationScope::Global => "global".to_string(),
            OrganizationScope::Organization(id) => format!("org.{}", id),
            OrganizationScope::Department(id) => format!("dept.{}", id),
            OrganizationScope::Team(id) => format!("team.{}", id),
            OrganizationScope::Role(id) => format!("role.{}", id),
            OrganizationScope::Location(loc) => format!("loc.{}", loc),
            OrganizationScope::Region(region) => format!("region.{}", region),
            OrganizationScope::Division(id) => format!("div.{}", id),
            OrganizationScope::Project(id) => format!("proj.{}", id),
            OrganizationScope::CostCenter(cc) => format!("cc.{}", cc),
            OrganizationScope::Vendor(id) => format!("vendor.{}", id),
        });
        
        // Add operation if present
        if let Some(operation) = &self.operation {
            parts.push(operation.clone());
        }
        
        // Add entity ID if present
        if let Some(entity_id) = &self.entity_id {
            parts.push(entity_id.clone());
        }
        
        // Add context parameters as key-value pairs
        for (key, value) in &self.context {
            parts.push(format!("{}={}", key, value));
        }
        
        parts.join(".")
    }
    
    /// Parses a NATS subject string into an OrganizationSubject
    pub fn from_subject_string(subject: &str) -> Result<Self, SubjectParseError> {
        let parts: Vec<&str> = subject.split('.').collect();
        
        if parts.len() < 4 {
            return Err(SubjectParseError::InsufficientParts);
        }
        
        let mut idx = 0;
        
        // Check for namespace (optional)
        let namespace = if parts.len() > 5 && !matches!(parts[0], "events" | "commands" | "queries" | "workflows" | "system" | "analytics" | "compliance" | "integration") {
            let ns = Some(parts[0].to_string());
            idx += 1;
            ns
        } else {
            None
        };
        
        // Parse root
        let root = match parts[idx] {
            "events" => OrganizationSubjectRoot::Events,
            "commands" => OrganizationSubjectRoot::Commands,
            "queries" => OrganizationSubjectRoot::Queries,
            "workflows" => OrganizationSubjectRoot::Workflows,
            "system" => OrganizationSubjectRoot::System,
            "analytics" => OrganizationSubjectRoot::Analytics,
            "compliance" => OrganizationSubjectRoot::Compliance,
            "integration" => OrganizationSubjectRoot::Integration,
            _ => return Err(SubjectParseError::InvalidRoot(parts[idx].to_string())),
        };
        idx += 1;
        
        // Verify domain
        if parts[idx] != "organization" {
            return Err(SubjectParseError::InvalidDomain(parts[idx].to_string()));
        }
        idx += 1;
        
        // Parse aggregate
        let aggregate = match parts[idx] {
            "organization" => OrganizationAggregate::Organization,
            "department" => OrganizationAggregate::Department,
            "team" => OrganizationAggregate::Team,
            "role" => OrganizationAggregate::Role,
            "policy" => OrganizationAggregate::Policy,
            "resource" => OrganizationAggregate::Resource,
            "structure" => OrganizationAggregate::Structure,
            "culture" => OrganizationAggregate::Culture,
            "strategy" => OrganizationAggregate::Strategy,
            "performance" => OrganizationAggregate::Performance,
            "communication" => OrganizationAggregate::Communication,
            "change" => OrganizationAggregate::Change,
            "risk" => OrganizationAggregate::Risk,
            "vendor" => OrganizationAggregate::Vendor,
            "location" => OrganizationAggregate::Location,
            _ => return Err(SubjectParseError::InvalidAggregate(parts[idx].to_string())),
        };
        idx += 1;
        
        // Parse scope
        if idx >= parts.len() {
            return Err(SubjectParseError::MissingScope);
        }
        
        let scope = if parts[idx] == "global" {
            OrganizationScope::Global
        } else if let Some((scope_type, scope_id)) = parts[idx].split_once('.') {
            match scope_type {
                "org" => OrganizationScope::Organization(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                "dept" => OrganizationScope::Department(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                "team" => OrganizationScope::Team(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                "role" => OrganizationScope::Role(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                "loc" => OrganizationScope::Location(scope_id.to_string()),
                "region" => OrganizationScope::Region(scope_id.to_string()),
                "div" => OrganizationScope::Division(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                "proj" => OrganizationScope::Project(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                "cc" => OrganizationScope::CostCenter(scope_id.to_string()),
                "vendor" => OrganizationScope::Vendor(
                    Uuid::parse_str(scope_id).map_err(|_| SubjectParseError::InvalidUuid(scope_id.to_string()))?
                ),
                _ => return Err(SubjectParseError::InvalidScope(parts[idx].to_string())),
            }
        } else {
            return Err(SubjectParseError::InvalidScope(parts[idx].to_string()));
        };
        idx += 1;
        
        // Parse operation (optional)
        let operation = if idx < parts.len() && !parts[idx].contains('=') {
            let op = Some(parts[idx].to_string());
            idx += 1;
            op
        } else {
            None
        };
        
        // Parse entity ID (optional)
        let entity_id = if idx < parts.len() && !parts[idx].contains('=') {
            let id = Some(parts[idx].to_string());
            idx += 1;
            id
        } else {
            None
        };
        
        // Parse context parameters
        let mut context = HashMap::new();
        while idx < parts.len() {
            if let Some((key, value)) = parts[idx].split_once('=') {
                context.insert(key.to_string(), value.to_string());
            }
            idx += 1;
        }
        
        Ok(Self {
            namespace,
            root,
            domain: "organization".to_string(),
            aggregate,
            scope,
            operation,
            entity_id,
            context,
        })
    }
    
    /// Creates a wildcard subject for subscribing to multiple related subjects
    pub fn to_wildcard_string(&self, wildcard_level: WildcardLevel) -> String {
        match wildcard_level {
            WildcardLevel::Operation => {
                format!("{}.{}.{}.{}.{}.*", 
                    self.namespace.as_deref().unwrap_or(""),
                    match &self.root {
                        OrganizationSubjectRoot::Events => "events",
                        OrganizationSubjectRoot::Commands => "commands",
                        OrganizationSubjectRoot::Queries => "queries",
                        OrganizationSubjectRoot::Workflows => "workflows",
                        OrganizationSubjectRoot::System => "system",
                        OrganizationSubjectRoot::Analytics => "analytics",
                        OrganizationSubjectRoot::Compliance => "compliance",
                        OrganizationSubjectRoot::Integration => "integration",
                    },
                    &self.domain,
                    match &self.aggregate {
                        OrganizationAggregate::Organization => "organization",
                        OrganizationAggregate::Department => "department",
                        OrganizationAggregate::Team => "team",
                        OrganizationAggregate::Role => "role",
                        OrganizationAggregate::Policy => "policy",
                        OrganizationAggregate::Resource => "resource",
                        OrganizationAggregate::Structure => "structure",
                        OrganizationAggregate::Culture => "culture",
                        OrganizationAggregate::Strategy => "strategy",
                        OrganizationAggregate::Performance => "performance",
                        OrganizationAggregate::Communication => "communication",
                        OrganizationAggregate::Change => "change",
                        OrganizationAggregate::Risk => "risk",
                        OrganizationAggregate::Vendor => "vendor",
                        OrganizationAggregate::Location => "location",
                    },
                    match &self.scope {
                        OrganizationScope::Global => "global",
                        OrganizationScope::Organization(id) => &format!("org.{}", id),
                        OrganizationScope::Department(id) => &format!("dept.{}", id),
                        OrganizationScope::Team(id) => &format!("team.{}", id),
                        OrganizationScope::Role(id) => &format!("role.{}", id),
                        OrganizationScope::Location(loc) => &format!("loc.{}", loc),
                        OrganizationScope::Region(region) => &format!("region.{}", region),
                        OrganizationScope::Division(id) => &format!("div.{}", id),
                        OrganizationScope::Project(id) => &format!("proj.{}", id),
                        OrganizationScope::CostCenter(cc) => &format!("cc.{}", cc),
                        OrganizationScope::Vendor(id) => &format!("vendor.{}", id),
                    }
                ).trim_start_matches('.').to_string()
            },
            WildcardLevel::Scope => {
                format!("{}.{}.{}.{}.*", 
                    self.namespace.as_deref().unwrap_or(""),
                    match &self.root {
                        OrganizationSubjectRoot::Events => "events",
                        OrganizationSubjectRoot::Commands => "commands",
                        OrganizationSubjectRoot::Queries => "queries",
                        OrganizationSubjectRoot::Workflows => "workflows",
                        OrganizationSubjectRoot::System => "system",
                        OrganizationSubjectRoot::Analytics => "analytics",
                        OrganizationSubjectRoot::Compliance => "compliance",
                        OrganizationSubjectRoot::Integration => "integration",
                    },
                    &self.domain,
                    match &self.aggregate {
                        OrganizationAggregate::Organization => "organization",
                        OrganizationAggregate::Department => "department",
                        OrganizationAggregate::Team => "team",
                        OrganizationAggregate::Role => "role",
                        OrganizationAggregate::Policy => "policy",
                        OrganizationAggregate::Resource => "resource",
                        OrganizationAggregate::Structure => "structure",
                        OrganizationAggregate::Culture => "culture",
                        OrganizationAggregate::Strategy => "strategy",
                        OrganizationAggregate::Performance => "performance",
                        OrganizationAggregate::Communication => "communication",
                        OrganizationAggregate::Change => "change",
                        OrganizationAggregate::Risk => "risk",
                        OrganizationAggregate::Vendor => "vendor",
                        OrganizationAggregate::Location => "location",
                    }
                ).trim_start_matches('.').to_string()
            },
            WildcardLevel::Aggregate => {
                format!("{}.{}.{}.*", 
                    self.namespace.as_deref().unwrap_or(""),
                    match &self.root {
                        OrganizationSubjectRoot::Events => "events",
                        OrganizationSubjectRoot::Commands => "commands",
                        OrganizationSubjectRoot::Queries => "queries",
                        OrganizationSubjectRoot::Workflows => "workflows",
                        OrganizationSubjectRoot::System => "system",
                        OrganizationSubjectRoot::Analytics => "analytics",
                        OrganizationSubjectRoot::Compliance => "compliance",
                        OrganizationSubjectRoot::Integration => "integration",
                    },
                    &self.domain
                ).trim_start_matches('.').to_string()
            },
            WildcardLevel::All => ">".to_string(),
        }
    }
}

/// Wildcard levels for NATS subscriptions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WildcardLevel {
    /// Wildcard at operation level
    Operation,
    
    /// Wildcard at scope level
    Scope,
    
    /// Wildcard at aggregate level
    Aggregate,
    
    /// Subscribe to everything
    All,
}

/// Organization-specific subject pattern builders for common operations
impl OrganizationSubject {
    // Organization lifecycle operations
    pub fn organization_created(org_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Organization,
            OrganizationScope::Global,
        )
        .with_operation("created".to_string())
        .with_entity_id(org_id.to_string())
    }
    
    pub fn organization_updated(org_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Organization,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("updated".to_string())
        .with_entity_id(org_id.to_string())
    }
    
    pub fn organization_merged(parent_org_id: Uuid, merged_org_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Organization,
            OrganizationScope::Organization(parent_org_id),
        )
        .with_operation("merged".to_string())
        .with_entity_id(merged_org_id.to_string())
        .with_context("parent_org".to_string(), parent_org_id.to_string())
    }
    
    pub fn organization_dissolved(org_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Organization,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("dissolved".to_string())
        .with_entity_id(org_id.to_string())
    }
    
    // Department management operations
    pub fn department_created(org_id: Uuid, dept_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Department,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("created".to_string())
        .with_entity_id(dept_id.to_string())
    }
    
    pub fn department_restructured(org_id: Uuid, dept_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Department,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("restructured".to_string())
        .with_entity_id(dept_id.to_string())
    }
    
    // Team management operations
    pub fn team_formed(org_id: Uuid, team_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Team,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("formed".to_string())
        .with_entity_id(team_id.to_string())
    }
    
    pub fn team_disbanded(org_id: Uuid, team_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Team,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("disbanded".to_string())
        .with_entity_id(team_id.to_string())
    }
    
    // Role and position management
    pub fn role_created(org_id: Uuid, role_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Role,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("created".to_string())
        .with_entity_id(role_id.to_string())
    }
    
    pub fn role_assignment_changed(org_id: Uuid, role_id: Uuid, person_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Role,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("assignment_changed".to_string())
        .with_entity_id(role_id.to_string())
        .with_context("person_id".to_string(), person_id.to_string())
    }
    
    // Policy management operations
    pub fn policy_created(org_id: Uuid, policy_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Policy,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("created".to_string())
        .with_entity_id(policy_id.to_string())
    }
    
    pub fn policy_updated(org_id: Uuid, policy_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Policy,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("updated".to_string())
        .with_entity_id(policy_id.to_string())
    }
    
    pub fn policy_violation_detected(org_id: Uuid, policy_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Policy,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("violation_detected".to_string())
        .with_entity_id(policy_id.to_string())
    }
    
    // Resource management operations
    pub fn resource_allocated(org_id: Uuid, resource_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Resource,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("allocated".to_string())
        .with_entity_id(resource_id.to_string())
    }
    
    pub fn resource_deallocated(org_id: Uuid, resource_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Resource,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("deallocated".to_string())
        .with_entity_id(resource_id.to_string())
    }
    
    // Strategic planning operations
    pub fn strategy_defined(org_id: Uuid, strategy_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Strategy,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("defined".to_string())
        .with_entity_id(strategy_id.to_string())
    }
    
    pub fn objective_achieved(org_id: Uuid, objective_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Strategy,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("objective_achieved".to_string())
        .with_entity_id(objective_id.to_string())
    }
    
    // Performance monitoring operations
    pub fn performance_measured(org_id: Uuid, metric_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Analytics,
            OrganizationAggregate::Performance,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("measured".to_string())
        .with_entity_id(metric_id.to_string())
    }
    
    pub fn kpi_threshold_exceeded(org_id: Uuid, kpi_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Performance,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("threshold_exceeded".to_string())
        .with_entity_id(kpi_id.to_string())
    }
    
    // Change management operations
    pub fn change_initiated(org_id: Uuid, change_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Change,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("initiated".to_string())
        .with_entity_id(change_id.to_string())
    }
    
    pub fn change_completed(org_id: Uuid, change_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Change,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("completed".to_string())
        .with_entity_id(change_id.to_string())
    }
    
    // Risk management operations
    pub fn risk_identified(org_id: Uuid, risk_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Risk,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("identified".to_string())
        .with_entity_id(risk_id.to_string())
    }
    
    pub fn risk_mitigated(org_id: Uuid, risk_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Risk,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("mitigated".to_string())
        .with_entity_id(risk_id.to_string())
    }
    
    // Compliance operations
    pub fn compliance_check_completed(org_id: Uuid, check_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Compliance,
            OrganizationAggregate::Organization,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("check_completed".to_string())
        .with_entity_id(check_id.to_string())
    }
    
    pub fn compliance_violation_reported(org_id: Uuid, violation_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Compliance,
            OrganizationAggregate::Organization,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("violation_reported".to_string())
        .with_entity_id(violation_id.to_string())
    }
    
    // Vendor management operations
    pub fn vendor_onboarded(org_id: Uuid, vendor_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Vendor,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("onboarded".to_string())
        .with_entity_id(vendor_id.to_string())
    }
    
    pub fn vendor_contract_renewed(org_id: Uuid, vendor_id: Uuid, contract_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Vendor,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("contract_renewed".to_string())
        .with_entity_id(vendor_id.to_string())
        .with_context("contract_id".to_string(), contract_id.to_string())
    }
    
    // Workflow orchestration patterns
    pub fn onboarding_workflow_started(org_id: Uuid, workflow_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Workflows,
            OrganizationAggregate::Organization,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("onboarding_started".to_string())
        .with_entity_id(workflow_id.to_string())
    }
    
    pub fn restructuring_workflow_completed(org_id: Uuid, workflow_id: Uuid) -> Self {
        Self::new(
            OrganizationSubjectRoot::Workflows,
            OrganizationAggregate::Structure,
            OrganizationScope::Organization(org_id),
        )
        .with_operation("restructuring_completed".to_string())
        .with_entity_id(workflow_id.to_string())
    }
}

impl Display for OrganizationSubject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_subject_string())
    }
}

/// Errors that can occur when parsing subject strings
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubjectParseError {
    InsufficientParts,
    InvalidRoot(String),
    InvalidDomain(String),
    InvalidAggregate(String),
    InvalidScope(String),
    InvalidUuid(String),
    MissingScope,
}

impl Display for SubjectParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SubjectParseError::InsufficientParts => write!(f, "Subject must have at least 4 parts"),
            SubjectParseError::InvalidRoot(root) => write!(f, "Invalid root pattern: {}", root),
            SubjectParseError::InvalidDomain(domain) => write!(f, "Invalid domain: {}", domain),
            SubjectParseError::InvalidAggregate(aggregate) => write!(f, "Invalid aggregate: {}", aggregate),
            SubjectParseError::InvalidScope(scope) => write!(f, "Invalid scope: {}", scope),
            SubjectParseError::InvalidUuid(uuid) => write!(f, "Invalid UUID: {}", uuid),
            SubjectParseError::MissingScope => write!(f, "Missing scope specification"),
        }
    }
}

impl std::error::Error for SubjectParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_subject_creation() {
        let subject = OrganizationSubject::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Organization,
            OrganizationScope::Global,
        );
        
        assert_eq!(subject.to_subject_string(), "events.organization.organization.global");
    }
    
    #[test]
    fn test_organization_created_subject() {
        let org_id = Uuid::now_v7();
        let subject = OrganizationSubject::organization_created(org_id);
        let subject_string = subject.to_subject_string();
        
        assert!(subject_string.starts_with("events.organization.organization.global.created"));
        assert!(subject_string.contains(&org_id.to_string()));
    }
    
    #[test]
    fn test_department_subject_with_context() {
        let org_id = Uuid::now_v7();
        let dept_id = Uuid::now_v7();
        let subject = OrganizationSubject::department_created(org_id, dept_id)
            .with_context("budget".to_string(), "1000000".to_string());
        
        let subject_string = subject.to_subject_string();
        assert!(subject_string.contains("budget=1000000"));
    }
    
    #[test]
    fn test_wildcard_subjects() {
        let subject = OrganizationSubject::new(
            OrganizationSubjectRoot::Events,
            OrganizationAggregate::Organization,
            OrganizationScope::Global,
        );
        
        let wildcard = subject.to_wildcard_string(WildcardLevel::Operation);
        assert_eq!(wildcard, "events.organization.organization.global.*");
    }
    
    #[test]
    fn test_subject_parsing() {
        let original = "events.organization.department.org.12345678-1234-5678-9012-123456789012.created.dept-456";
        let parsed = OrganizationSubject::from_subject_string(original);
        
        assert!(parsed.is_ok());
        let subject = parsed.unwrap();
        assert_eq!(subject.root, OrganizationSubjectRoot::Events);
        assert_eq!(subject.aggregate, OrganizationAggregate::Department);
        assert_eq!(subject.operation, Some("created".to_string()));
        assert_eq!(subject.entity_id, Some("dept-456".to_string()));
    }
    
    #[test]
    fn test_complex_workflow_subject() {
        let org_id = Uuid::now_v7();
        let workflow_id = Uuid::now_v7();
        let subject = OrganizationSubject::onboarding_workflow_started(org_id, workflow_id)
            .with_context("employee_type".to_string(), "full_time".to_string())
            .with_context("department".to_string(), "engineering".to_string());
        
        let subject_string = subject.to_subject_string();
        assert!(subject_string.contains("workflows.organization.organization"));
        assert!(subject_string.contains("onboarding_started"));
        assert!(subject_string.contains("employee_type=full_time"));
        assert!(subject_string.contains("department=engineering"));
    }
}