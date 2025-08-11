<!--
Copyright 2025 Cowboy AI, LLC.
SPDX-License-Identifier: MIT
-->

# CIM Domain: Organization Extension

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

> Organization domain extension for CIM Document system, providing specialized functionality for organizational structure management, governance, compliance, resource allocation, and strategic planning.

## Overview

The Organization domain extends the base `cim-domain-document` with specialized functionality for managing complex organizational structures, from startups to large enterprises. It follows the composition-over-inheritance pattern where `Organization` composes a base `Document` and adds organization-specific components and processing capabilities.

### Key Features

- 🏢 **Organizational Structure Management**: Complete lifecycle management of organizations, departments, teams, and roles
- 🎯 **Strategic Planning & Execution**: Goal setting, strategy definition, and performance tracking
- 📋 **Policy & Governance Framework**: Comprehensive policy management with automated compliance monitoring
- 💰 **Resource Allocation & Management**: Budget management, asset allocation, and financial planning
- 🔍 **Compliance & Audit Operations**: Regulatory compliance automation with audit trail capabilities
- 👥 **Team & Role Management**: Dynamic team formation, role assignment, and organizational restructuring
- 📊 **Analytics & Performance Measurement**: KPI tracking, performance analytics, and organizational health assessment
- 🔄 **Change Management**: Structured organizational transformation with stakeholder management

## Mathematical Foundation

The CIM Organization Domain is built on a formal [Organization Subject Algebra](docs/algebra/README.md) that provides:

- **Rigorous Mathematical Foundation**: 7-tuple algebraic structure with proven properties
- **Type Safety**: Compile-time guarantees for organizational management operations  
- **Compositional Operations**: Sequential (⊕), Parallel (⊗), and Conditional (→) transformations
- **Governance Constraints**: Algebraic compliance rules ensuring regulatory adherence
- **Distributed Processing**: NATS-based implementation of algebraic operations

### Key Algebraic Operations

```
Sequential:   create_organization ⊕ establish_departments ⊕ assign_leadership  
Parallel:     policy_creation ⊗ resource_allocation ⊗ compliance_setup
Conditional:  budget_request →[exceeds_threshold] require_executive_approval
```

See [Organization Subject Algebra Documentation](docs/algebra/README.md) for complete mathematical definitions.

## Architecture

### Organization Profile Composition

```mermaid
classDiagram
    class Organization {
        +Document document
        +Option~OrganizationInfoComponent~ org_info
        +Option~StructureComponent~ structure
        +Option~ResourceComponent~ resources  
        +Option~PolicyComponent~ policies
        +Option~PerformanceComponent~ performance
        +Option~ComplianceComponent~ compliance
        +Option~StrategyComponent~ strategy
        +Option~CultureComponent~ culture
        
        +new(id: EntityId, name: String) Organization
        +create_department(dept: Department)
        +allocate_resources(allocation: ResourceAllocation)
        +enforce_policy(policy_id: Uuid)
        +organization_summary() OrganizationSummary
    }
    
    class Document {
        +Entity~DocumentMarker~ entity
        +Option~DocumentInfoComponent~ info
        +Option~ContentAddressComponent~ content_address
        +Option~ClassificationComponent~ classification
    }
    
    class OrganizationInfoComponent {
        +String name
        +String legal_name
        +OrganizationType organization_type
        +Industry industry
        +OrganizationSize size
        +DateTime~Utc~ founding_date
        +Location headquarters
        +RegistrationDetails registration
    }
    
    class StructureComponent {
        +Vec~Department~ departments
        +Vec~Team~ teams
        +Vec~Role~ roles
        +OrganizationalChart chart
        +GovernanceModel governance
        +Vec~ReportingLine~ reporting_lines
    }
    
    class ResourceComponent {
        +BudgetAllocation budget
        +Vec~Asset~ assets
        +Vec~ResourcePool~ resource_pools
        +Vec~CostCenter~ cost_centers
        +FinancialAccounts accounts
    }
    
    class PolicyComponent {
        +Vec~Policy~ policies
        +Vec~Procedure~ procedures
        +Vec~ComplianceFramework~ frameworks
        +Vec~AuditEvent~ audit_trail
        +Vec~RiskAssessment~ risk_assessments
    }
    
    Organization --> Document
    Organization --> OrganizationInfoComponent
    Organization --> StructureComponent
    Organization --> ResourceComponent
    Organization --> PolicyComponent
```

### Organization Management Pipeline

```mermaid
graph TB
    subgraph "Organization Formation"
        CREATE[Create Organization]
        VALIDATE[Legal Validation]
        REGISTER[Registration Process]
        STRUCTURE[Initial Structure Setup]
    end
    
    subgraph "Governance Setup"
        POLICIES[Policy Framework]
        COMPLIANCE[Compliance Setup] 
        GOVERNANCE[Governance Model]
        AUDIT[Audit Framework]
    end
    
    subgraph "Operational Structure"
        DEPTS[Department Creation]
        TEAMS[Team Formation]
        ROLES[Role Definition]
        HIERARCHY[Reporting Structure]
    end
    
    subgraph "Resource Management"
        BUDGET[Budget Allocation]
        ASSETS[Asset Management]
        FACILITIES[Facilities Setup]
        SYSTEMS[System Provisioning]
    end
    
    subgraph "Strategic Planning"
        STRATEGY[Strategy Definition]
        OBJECTIVES[Goal Setting]
        METRICS[KPI Framework]
        PERFORMANCE[Performance Tracking]
    end
    
    CREATE --> VALIDATE
    VALIDATE --> REGISTER
    REGISTER --> STRUCTURE
    
    STRUCTURE --> POLICIES
    POLICIES --> COMPLIANCE
    COMPLIANCE --> GOVERNANCE
    GOVERNANCE --> AUDIT
    
    STRUCTURE --> DEPTS
    DEPTS --> TEAMS
    TEAMS --> ROLES
    ROLES --> HIERARCHY
    
    STRUCTURE --> BUDGET
    BUDGET --> ASSETS
    ASSETS --> FACILITIES
    FACILITIES --> SYSTEMS
    
    GOVERNANCE --> STRATEGY
    STRATEGY --> OBJECTIVES
    OBJECTIVES --> METRICS
    METRICS --> PERFORMANCE
    
    style CREATE fill:#ff9999
    style GOVERNANCE fill:#99ccff
    style STRATEGY fill:#99ff99
```

## Usage Examples

### Basic Organization Creation

```rust
use cim_domain_organization::prelude::*;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handler = OrganizationCommandHandler::new();
    
    // Create an organization
    let org_id = EntityId::new();
    
    let create_command = CreateOrganization::new(
        org_id,
        "Acme Corporation".to_string(),
        "Acme Corporation Inc.".to_string(),
    )
    .with_organization_type(OrganizationType::Corporation)
    .with_industry(Industry::Technology)
    .with_size(OrganizationSize::Medium)
    .with_headquarters(Location {
        address: "123 Tech Street".to_string(),
        city: "San Francisco".to_string(),
        state: "CA".to_string(),
        country: "US".to_string(),
        postal_code: "94105".to_string(),
    })
    .with_founding_date(NaiveDate::from_ymd_opt(2020, 1, 15).unwrap())
    .with_initial_budget(Money::new(10_000_000, Currency::USD));
    
    let events = handler.handle_create_organization(create_command).await?;
    println!("Created organization with {} events", events.len());
    
    Ok(())
}
```

### Department Structure Management

```rust
use cim_domain_organization::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let department_service = DefaultDepartmentService::new();
    let org_id = EntityId::new();
    
    // Create engineering department
    let engineering_dept = Department::new(
        org_id,
        "Engineering".to_string(),
        "Software development and engineering operations".to_string(),
        DepartmentType::Operational,
    )
    .with_head_of_department(PersonId::new(), "VP of Engineering".to_string())
    .with_budget_allocation(BudgetAllocation::new(5_000_000, Currency::USD))
    .with_location("San Francisco HQ".to_string())
    .with_objectives(vec![
        "Deliver high-quality software products".to_string(),
        "Maintain 99.9% system uptime".to_string(),
        "Scale engineering team by 50%".to_string(),
    ]);
    
    let dept_result = department_service
        .create_department(engineering_dept)
        .await?;
        
    println!("Department created: {}", dept_result.department_id);
    
    // Create sub-teams
    let platform_team = Team::new(
        dept_result.department_id,
        "Platform Engineering Team".to_string(),
        TeamType::Permanent,
    )
    .with_purpose("Build and maintain core platform infrastructure".to_string())
    .with_team_lead(PersonId::new(), "Senior Engineering Manager".to_string())
    .with_target_size(8)
    .with_budget(Money::new(1_200_000, Currency::USD));
    
    let team_result = department_service
        .form_team(platform_team)
        .await?;
        
    println!("Team formed: {}", team_result.team_id);
    
    Ok(())
}
```

### Policy and Compliance Management

```rust
use cim_domain_organization::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let policy_service = DefaultPolicyService::new();
    let org_id = EntityId::new();
    
    // Create code of conduct policy
    let code_of_conduct = Policy::new(
        org_id,
        "Code of Conduct Policy".to_string(),
        PolicyType::Behavioral,
        PolicyCategory::WorkplaceConduct,
    )
    .with_version("1.0".to_string())
    .with_effective_date(DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")?.with_timezone(&Utc))
    .with_scope(PolicyScope::AllEmployees)
    .with_policy_content(PolicyContent {
        summary: "Guidelines for professional conduct and workplace behavior".to_string(),
        detailed_policy_url: "https://policies.company.com/code-of-conduct-v1.0".to_string(),
        key_requirements: vec![
            "Treat all colleagues with respect".to_string(),
            "Maintain confidentiality of sensitive information".to_string(),
            "Report violations through proper channels".to_string(),
            "Comply with all applicable laws and regulations".to_string(),
        ],
    })
    .with_enforcement(EnforcementConfig {
        violation_reporting: ReportingMethod::AnonymousHotline,
        investigation_process: InvestigationProcess::HrLed,
        disciplinary_actions: vec![
            DisciplinaryAction::VerbalWarning,
            DisciplinaryAction::WrittenWarning,
            DisciplinaryAction::Suspension,
            DisciplinaryAction::Termination,
        ],
    })
    .with_training_requirement(TrainingRequirement {
        required: true,
        frequency: TrainingFrequency::Annual,
        completion_deadline: Duration::days(30),
    });
    
    let policy_result = policy_service
        .create_policy(code_of_conduct)
        .await?;
        
    println!("Policy created: {}", policy_result.policy_id);
    
    // Monitor policy compliance
    let compliance_monitor = ComplianceMonitor::new()
        .with_policy(policy_result.policy_id)
        .with_monitoring_frequency(MonitoringFrequency::Continuous)
        .with_violation_detection(ViolationDetection::Automated);
    
    compliance_monitor.start_monitoring().await?;
    
    Ok(())
}
```

### Resource Allocation and Management

```rust
use cim_domain_organization::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_service = DefaultResourceService::new();
    let org_id = EntityId::new();
    let dept_id = EntityId::new();
    
    // Allocate budget to department
    let budget_allocation = ResourceAllocation::new(
        org_id,
        ResourceType::Budget,
        AllocationTarget::Department(dept_id),
    )
    .with_resource_details(ResourceDetails::Budget {
        amount: Money::new(2_500_000, Currency::USD),
        allocation_period: AllocationPeriod::Quarterly,
        purpose: "Operational expenses for Q1 2025".to_string(),
    })
    .with_allocation_breakdown(AllocationBreakdown {
        personnel: Percentage::new(60.0),
        technology: Percentage::new(25.0),
        marketing: Percentage::new(10.0),
        operations: Percentage::new(5.0),
    })
    .with_constraints(AllocationConstraints {
        spending_approval_threshold: Money::new(50_000, Currency::USD),
        reallocation_requires_approval: true,
        reporting_frequency: ReportingFrequency::Monthly,
    });
    
    let allocation_result = resource_service
        .allocate_resource(budget_allocation)
        .await?;
        
    println!("Resource allocated: {}", allocation_result.allocation_id);
    
    // Request additional resources
    let resource_request = ResourceRequest::new(
        dept_id,
        ResourceType::AdditionalHeadcount,
    )
    .with_request_details(RequestDetails::Headcount {
        headcount_increase: 5,
        roles_requested: vec![
            RoleRequest {
                title: "Senior Software Engineer".to_string(),
                count: 3,
                urgency: RequestUrgency::High,
            },
            RoleRequest {
                title: "Product Manager".to_string(),
                count: 2,
                urgency: RequestUrgency::Medium,
            },
        ],
    })
    .with_business_justification(BusinessJustification {
        reason: "Increased product demand".to_string(),
        expected_roi: Percentage::new(300.0),
        timeline: "Q2 2025".to_string(),
        risk_if_not_approved: "Missed revenue targets".to_string(),
    })
    .with_budget_impact(Money::new(850_000, Currency::USD))
    .with_priority(RequestPriority::High);
    
    let request_result = resource_service
        .submit_resource_request(resource_request)
        .await?;
        
    println!("Resource request submitted: {}", request_result.request_id);
    
    Ok(())
}
```

### Strategic Planning and Performance Management

```rust
use cim_domain_organization::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let strategy_service = DefaultStrategyService::new();
    let performance_service = DefaultPerformanceService::new();
    let org_id = EntityId::new();
    
    // Define organizational strategy
    let strategy = OrganizationalStrategy::new(
        org_id,
        "Digital Transformation 2025".to_string(),
        StrategyType::Digital,
    )
    .with_vision("Become the leading digital-first organization in our industry".to_string())
    .with_strategic_objectives(vec![
        StrategicObjective::new(
            "Digital Customer Experience".to_string(),
            "Digitize 100% of customer touchpoints".to_string(),
            ObjectiveType::CustomerFocused,
        ).with_target_metrics(vec![
            TargetMetric::new("digital_adoption_rate", 95.0, MetricUnit::Percentage),
            TargetMetric::new("customer_satisfaction", 4.5, MetricUnit::Score),
        ]),
        StrategicObjective::new(
            "Operational Excellence".to_string(),
            "Achieve operational efficiency through automation".to_string(),
            ObjectiveType::Operational,
        ).with_target_metrics(vec![
            TargetMetric::new("process_automation", 80.0, MetricUnit::Percentage),
            TargetMetric::new("cost_reduction", 15.0, MetricUnit::Percentage),
        ]),
    ])
    .with_timeline(StrategyTimeline {
        start_date: DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")?.with_timezone(&Utc),
        end_date: DateTime::parse_from_rfc3339("2025-12-31T23:59:59Z")?.with_timezone(&Utc),
        milestones: vec![
            Milestone::new("Q1: Foundation Phase", "2025-03-31"),
            Milestone::new("Q2: Implementation Phase", "2025-06-30"),
            Milestone::new("Q3: Optimization Phase", "2025-09-30"),
            Milestone::new("Q4: Excellence Phase", "2025-12-31"),
        ],
    });
    
    let strategy_result = strategy_service
        .define_strategy(strategy)
        .await?;
        
    println!("Strategy defined: {}", strategy_result.strategy_id);
    
    // Track performance metrics
    let performance_measurement = PerformanceMeasurement::new(
        org_id,
        MeasurementScope::Organization,
    )
    .with_metrics(vec![
        PerformanceMetric::new("employee_satisfaction", 4.2, MetricScale::OneToFive),
        PerformanceMetric::new("productivity_index", 87.5, MetricScale::ZeroToHundred),
        PerformanceMetric::new("retention_rate", 94.2, MetricScale::Percentage),
        PerformanceMetric::new("innovation_index", 76.8, MetricScale::ZeroToHundred),
    ])
    .with_reporting_period(ReportingPeriod::Quarterly)
    .with_benchmarks(BenchmarkData {
        industry_average: 82.1,
        company_target: 90.0,
        previous_period: 85.3,
    });
    
    let measurement_result = performance_service
        .record_measurement(performance_measurement)
        .await?;
        
    println!("Performance measured: {}", measurement_result.measurement_id);
    
    // Generate organizational health assessment
    let health_assessment = performance_service
        .assess_organizational_health(org_id)
        .await?;
        
    println!("Organizational Health Assessment:");
    println!("- Overall Score: {:.1}/10", health_assessment.overall_score);
    println!("- Financial Health: {:.1}/10 ({})", 
             health_assessment.dimensions.financial_health.score,
             health_assessment.dimensions.financial_health.trend);
    println!("- Employee Engagement: {:.1}/10 ({})",
             health_assessment.dimensions.employee_engagement.score,
             health_assessment.dimensions.employee_engagement.trend);
    
    Ok(())
}
```

### Organizational Change Management

```rust
use cim_domain_organization::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let change_service = DefaultChangeService::new();
    let org_id = EntityId::new();
    
    // Initiate organizational restructuring
    let change_request = ChangeRequest::new(
        org_id,
        "Digital Transformation Restructuring".to_string(),
        ChangeType::Restructuring,
    )
    .with_description("Reorganize teams to support digital-first operations".to_string())
    .with_scope(ChangeScope {
        affected_departments: vec!["engineering", "product", "marketing"],
        affected_employees: 125,
        systems_impacted: vec!["hr_system", "org_chart", "access_controls"],
        budget_impact: Money::new(500_000, Currency::USD),
    })
    .with_business_justification(ChangeJustification {
        drivers: vec![
            "Market demands for digital services",
            "Competitive advantage through agility",
            "Operational efficiency improvements",
        ],
        expected_benefits: vec![
            "30% faster product delivery",
            "Improved cross-functional collaboration", 
            "Enhanced customer experience",
        ],
        risks_if_not_implemented: vec![
            "Loss of market share",
            "Decreased employee satisfaction",
            "Operational inefficiencies",
        ],
    })
    .with_timeline(ChangeTimeline {
        planning_phase: Duration::weeks(2),
        communication_phase: Duration::weeks(1),
        implementation_phase: Duration::weeks(4),
        stabilization_phase: Duration::weeks(2),
    });
    
    let change_result = change_service
        .initiate_change(change_request)
        .await?;
        
    println!("Change initiative started: {}", change_result.change_id);
    
    // Conduct impact assessment
    let impact_assessment = change_service
        .assess_change_impact(change_result.change_id)
        .await?;
        
    println!("Impact Assessment Results:");
    println!("- Employee Impact: {:?}", impact_assessment.employee_impact);
    println!("- System Changes: {:?}", impact_assessment.system_changes);
    println!("- Risk Level: {:?}", impact_assessment.overall_risk);
    
    // Create change management workflow
    let change_workflow = ChangeWorkflow::new(change_result.change_id)
        .add_phase(ChangePhase::Planning {
            stakeholder_analysis: true,
            communication_strategy: true,
            risk_mitigation_plan: true,
        })
        .add_phase(ChangePhase::Communication {
            town_halls: 3,
            training_sessions: 8,
            feedback_collection: true,
        })
        .add_phase(ChangePhase::Implementation {
            phased_rollout: true,
            pilot_groups: vec!["engineering_team_alpha"],
            monitoring_frequency: MonitoringFrequency::Daily,
        })
        .add_phase(ChangePhase::Stabilization {
            performance_monitoring: Duration::weeks(4),
            feedback_analysis: true,
            lessons_learned: true,
        });
    
    let workflow_result = change_service
        .execute_change_workflow(change_workflow)
        .await?;
        
    println!("Change workflow initiated: {}", workflow_result.workflow_id);
    
    Ok(())
}
```

## Organization Components

### OrganizationInfoComponent

Core organizational identity and legal information:

```rust
pub struct OrganizationInfoComponent {
    pub name: String,
    pub legal_name: String,
    pub organization_type: OrganizationType,
    pub industry: Industry,
    pub size: OrganizationSize,
    pub founding_date: DateTime<Utc>,
    pub headquarters: Location,
    pub registration_details: RegistrationDetails,
    
    // Financial summary
    pub annual_revenue: Option<Money>,
    pub annual_budget: Option<Money>,
    pub employee_count: u32,
    
    // Status and lifecycle
    pub status: OrganizationStatus,
    pub lifecycle_stage: LifecycleStage,
}
```

### StructureComponent

Organizational hierarchy and reporting relationships:

```rust
pub struct StructureComponent {
    pub departments: Vec<Department>,
    pub teams: Vec<Team>,
    pub roles: Vec<Role>,
    pub reporting_lines: Vec<ReportingLine>,
    pub organizational_chart: OrganizationalChart,
    pub governance_model: GovernanceModel,
    pub decision_matrix: DecisionMatrix,
}

impl StructureComponent {
    pub fn get_hierarchy(&self) -> OrganizationalHierarchy;
    pub fn find_reporting_chain(&self, role_id: Uuid) -> Vec<Role>;
    pub fn calculate_span_of_control(&self, manager_id: Uuid) -> u32;
    pub fn validate_structure_consistency(&self) -> StructureValidationResult;
}
```

### ResourceComponent

Financial and physical resource management:

```rust
pub struct ResourceComponent {
    pub budget_allocation: BudgetAllocation,
    pub financial_accounts: Vec<Account>,
    pub assets: Vec<Asset>,
    pub resource_pools: Vec<ResourcePool>,
    pub cost_centers: Vec<CostCenter>,
    pub resource_utilization: ResourceUtilization,
}

impl ResourceComponent {
    pub fn calculate_total_budget(&self) -> Money;
    pub fn get_available_budget(&self, period: TimePeriod) -> Money;
    pub fn track_resource_utilization(&self) -> UtilizationReport;
    pub fn forecast_resource_needs(&self, horizon: Duration) -> ResourceForecast;
}
```

### PolicyComponent

Governance, compliance, and policy management:

```rust
pub struct PolicyComponent {
    pub policies: Vec<Policy>,
    pub procedures: Vec<Procedure>,
    pub compliance_frameworks: Vec<ComplianceFramework>,
    pub audit_trail: Vec<AuditEvent>,
    pub risk_assessments: Vec<RiskAssessment>,
    pub governance_rules: Vec<GovernanceRule>,
}

impl PolicyComponent {
    pub fn check_compliance(&self, framework: ComplianceFramework) -> ComplianceStatus;
    pub fn enforce_policy(&self, policy_id: Uuid, context: EnforcementContext) -> EnforcementResult;
    pub fn generate_audit_report(&self, period: TimePeriod) -> AuditReport;
    pub fn assess_policy_effectiveness(&self) -> PolicyEffectivenessReport;
}
```

## Governance & Compliance

The Organization domain includes comprehensive governance and compliance features:

### Regulatory Compliance Features
- **Multi-Framework Support**: SOX, GDPR, HIPAA, ISO 27001, OSHA compliance
- **Automated Monitoring**: Continuous compliance checking with real-time alerts
- **Audit Trail**: Complete traceability of all organizational changes
- **Risk Assessment**: Continuous risk monitoring with mitigation planning
- **Regulatory Reporting**: Automated generation of compliance reports

### Governance Configuration

```rust
let governance_settings = GovernanceSettings {
    compliance_frameworks: vec![
        ComplianceFramework::SOX,
        ComplianceFramework::GDPR,
        ComplianceFramework::ISO27001,
    ],
    audit_frequency: AuditFrequency::Quarterly,
    risk_tolerance: RiskTolerance::Conservative,
    approval_hierarchies: ApprovalHierarchies::default(),
    segregation_of_duties: true,
    financial_controls: FinancialControls::Strict,
};
```

## Workflow System

Pre-defined workflows for organizational operations:

### Organization Formation Workflow
```rust
let formation_workflow = OrganizationFormationWorkflow::new()
    .add_step(LegalSetupStep::new())
    .add_step(RegistrationStep::new())
    .add_step(InitialStructureStep::new())
    .add_step(PolicyFrameworkStep::new())
    .add_step(ComplianceSetupStep::new())
    .add_step(ResourceAllocationStep::new())
    .add_step(ActivationStep::new());

let result = formation_workflow.execute(formation_data).await?;
```

### Available Workflows
- **Organization Formation**: Complete legal entity setup and activation
- **Department Restructuring**: Organizational structure changes and team transitions
- **Strategic Planning**: Strategy definition, goal setting, and execution tracking
- **Compliance Auditing**: Regulatory compliance assessment and reporting
- **Change Management**: Organizational transformation with stakeholder management
- **Performance Management**: KPI tracking, assessment, and improvement planning
- **Risk Management**: Risk identification, assessment, and mitigation
- **Policy Lifecycle**: Policy creation, review, approval, and enforcement

## Configuration

### Feature Flags

Optional dependencies can be enabled through Cargo features:

```toml
[dependencies]
cim-domain-organization = { version = "0.5.0", features = ["compliance", "analytics", "governance"] }
```

Available features:
- `compliance`: Enable comprehensive compliance and audit features
- `analytics`: Enable advanced analytics and performance measurement
- `governance`: Enable advanced governance and policy management
- `strategic-planning`: Enable strategic planning and execution tracking
- `change-management`: Enable organizational change management capabilities
- `full`: Enable all features

### Governance & Compliance Settings

Configure governance and compliance parameters:

```rust
let settings = OrganizationDomainSettings {
    legal_validation_timeout: Duration::from_secs(600),
    max_organizational_depth: 8,
    budget_approval_threshold: Money::new(100_000, Currency::USD),
    require_dual_authorization: true,
    audit_retention_period: Duration::days(2555), // 7 years
    compliance_monitoring_interval: Duration::hours(1),
    risk_assessment_frequency: Duration::days(30),
    governance_mode: GovernanceMode::Strict,
};
```

## Examples

See the [examples](examples/) directory for complete working examples:

- [`organization_lifecycle.rs`](examples/organization_lifecycle.rs) - Complete organization management workflow
- [`department_management.rs`](examples/department_management.rs) - Department creation and restructuring
- [`resource_allocation.rs`](examples/resource_allocation.rs) - Budget and resource management
- [`policy_compliance.rs`](examples/policy_compliance.rs) - Policy management and compliance monitoring
- [`strategic_planning.rs`](examples/strategic_planning.rs) - Strategic planning and performance tracking
- [`change_management.rs`](examples/change_management.rs) - Organizational change and transformation
- [`governance_framework.rs`](examples/governance_framework.rs) - Governance and audit operations

## Development

### Prerequisites

```bash
# Enter development shell
nix develop

# Install optional dependencies for full functionality
sudo apt-get install libssl-dev          # For legal validation APIs
sudo apt-get install libsqlite3-dev      # For local compliance data
sudo apt-get install libxml2-dev         # For regulatory document processing
```

### Testing

```bash
# Run all tests
cargo test

# Run with specific features
cargo test --features "compliance,analytics,governance"

# Run integration tests
cargo test --test "*_integration_test"

# Run compliance tests
cargo test --test "compliance_*" --features compliance

# Run governance tests
cargo test --test "governance_*" --features governance
```

### Building

```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release --features full
```

## Documentation

### Core Documentation
- [Organization Subject Algebra](docs/algebra/README.md) - Mathematical foundations
- [NATS API Reference](docs/api/README.md) - Complete API documentation
- [Cross-Reference Index](docs/cross-reference.md) - Links between concepts and implementation

### Integration Guides
- [Configuration Guide](docs/configuration/README.md) - Service configuration
- [Deployment Guide](docs/deployment/README.md) - Production deployment
- [Governance & Compliance Guide](docs/compliance/README.md) - Regulatory compliance
- [Security Best Practices](docs/security/README.md) - Security model

### Workflow Documentation
- [Formation Workflows](docs/workflows/formation/README.md) - Organization setup
- [Governance Workflows](docs/workflows/governance/README.md) - Governance operations
- [Compliance Workflows](docs/workflows/compliance/README.md) - Regulatory compliance
- [Change Management Workflows](docs/workflows/change/README.md) - Organizational transformation

### Specialized Guides
- [Strategic Planning](docs/strategy/README.md) - Strategic planning framework
- [Performance Management](docs/performance/README.md) - Performance measurement and optimization
- [Risk Management](docs/risk/README.md) - Enterprise risk management
- [Financial Management](docs/financial/README.md) - Budget and resource management

## License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.

## Copyright

Copyright 2025 Cowboy AI, LLC.