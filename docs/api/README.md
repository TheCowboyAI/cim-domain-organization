<!--
Copyright 2025 Cowboy AI, LLC.
SPDX-License-Identifier: MIT
-->

# CIM Organization Domain NATS API Documentation

## Table of Contents

- [Overview](#overview)
- [NATS Subject Architecture](#nats-subject-architecture)
- [Command Messages](#command-messages)
- [Event Messages](#event-messages)
- [Query Messages](#query-messages)
- [Workflow Messages](#workflow-messages)
- [Analytics Messages](#analytics-messages)
- [Compliance Messages](#compliance-messages)
- [Message Patterns](#message-patterns)
- [Error Handling](#error-handling)
- [Subject Algebra](#subject-algebra)

## Overview

The CIM Organization Domain provides a comprehensive NATS-only API for managing organizational structures, departments, teams, roles, policies, resources, and governance within the CIM ecosystem. All interactions occur through NATS message passing with mandatory correlation/causation tracking and follows CIM Organization Subject Algebra patterns for consistent message routing and processing.

## NATS Subject Architecture

The Organization Domain follows CIM Subject Algebra with hierarchical subject structure:

```
cim.organization.{version}.{operation}.{aggregate}.{scope}.{action}
```

### Subject Hierarchy

- **Domain**: `cim.organization`
- **Version**: `v1` (current version)
- **Operations**: `command`, `event`, `query`, `workflow`, `analytics`, `compliance`, `integration`
- **Aggregates**: `organization`, `department`, `team`, `role`, `policy`, `resource`, `structure`, `culture`, `strategy`, `performance`, `communication`, `change`, `risk`, `vendor`, `location`
- **Scopes**: Global, organization-specific, department-specific, team-specific, etc.
- **Actions**: Specific operations (create, update, merge, dissolve, etc.)

### Example Subjects

```
cim.organization.v1.command.organization.create
cim.organization.v1.command.department.org.{org_id}.create
cim.organization.v1.command.role.dept.{dept_id}.assign
cim.organization.v1.event.organization.created
cim.organization.v1.event.department.restructured
cim.organization.v1.query.organization.get
cim.organization.v1.workflow.onboarding.start
cim.organization.v1.analytics.performance.measure
cim.organization.v1.compliance.audit.initiate
```

## Command Messages

All commands are sent as NATS messages with JSON payloads and mandatory CIM message envelope.

### Message Envelope

Every message includes the CIM message envelope:

```json
{
  "correlation_id": "uuid-v4",
  "causation_id": "uuid-v4", 
  "message_id": "uuid-v4",
  "timestamp": "2025-01-15T10:30:00Z",
  "version": "1.0",
  "subject": "cim.organization.v1.command.organization.create",
  "payload": { ... }
}
```

### Organization Commands

#### Create Organization
**Subject**: `cim.organization.v1.command.organization.create`
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "name": "Acme Corporation",
  "legal_name": "Acme Corporation Inc.",
  "organization_type": "corporation",
  "industry": "technology",
  "size": "medium",
  "headquarters": {
    "address": "123 Tech Street",
    "city": "San Francisco",
    "state": "CA",
    "country": "US",
    "postal_code": "94105"
  },
  "registration_details": {
    "jurisdiction": "Delaware",
    "registration_number": "REG-123456789",
    "tax_id": "TAX-987654321"
  },
  "founding_date": "2020-01-15",
  "governance_model": "hierarchical",
  "initial_budget": {
    "amount": 10000000,
    "currency": "USD"
  }
}
```

#### Update Organization
**Subject**: `cim.organization.v1.command.organization.update`
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "updates": {
    "name": "Acme Technologies Inc.",
    "size": "large",
    "headquarters": {
      "address": "456 Innovation Drive",
      "city": "Austin",
      "state": "TX"
    }
  },
  "effective_date": "2025-02-01T00:00:00Z",
  "updated_by": "executive-user-id",
  "approval_required": true
}
```

#### Merge Organizations
**Subject**: `cim.organization.v1.command.organization.merge`
```json
{
  "parent_organization_id": "parent-org-id",
  "acquired_organization_id": "acquired-org-id",
  "merger_type": "acquisition",
  "effective_date": "2025-03-01T00:00:00Z",
  "transaction_details": {
    "value": 50000000,
    "currency": "USD",
    "structure": "cash_and_stock"
  },
  "integration_plan": {
    "timeline": "6_months",
    "key_milestones": [
      "legal_closure",
      "system_integration", 
      "team_integration",
      "culture_integration"
    ]
  },
  "approved_by": "board-of-directors"
}
```

#### Dissolve Organization
**Subject**: `cim.organization.v1.command.organization.dissolve`
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "dissolution_type": "voluntary",
  "effective_date": "2025-06-30T23:59:59Z",
  "reason": "strategic_restructuring",
  "asset_distribution_plan": {
    "cash_distribution": "shareholders",
    "asset_transfer": "parent_company",
    "employee_transition": "transfer_or_severance"
  },
  "legal_requirements": {
    "regulatory_filings": "required",
    "creditor_notifications": "required",
    "tax_clearance": "required"
  },
  "authorized_by": "board-resolution-id"
}
```

### Department Commands

#### Create Department
**Subject**: `cim.organization.v1.command.department.org.{org_id}.create`
```json
{
  "id": "dept-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "name": "Engineering",
  "description": "Software development and engineering operations",
  "department_type": "operational",
  "parent_department_id": null,
  "head_of_department": {
    "person_id": "manager-uuid",
    "title": "VP of Engineering",
    "start_date": "2025-01-15"
  },
  "budget_allocation": {
    "annual_budget": 5000000,
    "currency": "USD",
    "budget_category": "operational"
  },
  "location": {
    "primary_location": "San Francisco HQ",
    "remote_allowed": true,
    "hybrid_model": true
  },
  "objectives": [
    "Deliver high-quality software products",
    "Maintain 99.9% system uptime",
    "Scale engineering team by 50%"
  ]
}
```

#### Restructure Department
**Subject**: `cim.organization.v1.command.department.org.{org_id}.restructure`
```json
{
  "department_id": "dept-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "restructure_type": "split",
  "new_structure": {
    "frontend_engineering": {
      "teams": ["ui_team", "ux_team", "mobile_team"],
      "budget_percentage": 40
    },
    "backend_engineering": {
      "teams": ["api_team", "database_team", "infrastructure_team"],
      "budget_percentage": 60
    }
  },
  "effective_date": "2025-04-01T00:00:00Z",
  "impact_assessment": {
    "employees_affected": 45,
    "budget_reallocation": true,
    "systems_integration_required": true
  },
  "approved_by": "cto-user-id"
}
```

### Team Commands

#### Form Team
**Subject**: `cim.organization.v1.command.team.dept.{dept_id}.form`
```json
{
  "id": "team-uuid",
  "department_id": "dept-uuid",
  "name": "Platform Engineering Team",
  "team_type": "permanent",
  "purpose": "Build and maintain core platform infrastructure",
  "team_lead": {
    "person_id": "lead-uuid",
    "title": "Senior Engineering Manager"
  },
  "initial_members": [
    {
      "person_id": "engineer-1-uuid",
      "role": "senior_platform_engineer"
    },
    {
      "person_id": "engineer-2-uuid", 
      "role": "devops_engineer"
    }
  ],
  "target_size": 8,
  "formation_date": "2025-01-15",
  "budget": {
    "annual_budget": 1200000,
    "currency": "USD"
  },
  "success_metrics": [
    "Platform uptime > 99.9%",
    "Deployment frequency > 10/day",
    "Lead time < 2 hours"
  ]
}
```

#### Disband Team
**Subject**: `cim.organization.v1.command.team.dept.{dept_id}.disband`
```json
{
  "team_id": "team-uuid",
  "department_id": "dept-uuid",
  "disbandment_reason": "project_completion",
  "effective_date": "2025-03-31T23:59:59Z",
  "member_transition_plan": {
    "reassignments": [
      {
        "person_id": "engineer-1-uuid",
        "new_team_id": "new-team-uuid",
        "transition_date": "2025-04-01"
      }
    ],
    "knowledge_transfer": {
      "documentation_required": true,
      "handover_sessions": 5,
      "completion_deadline": "2025-03-25"
    }
  },
  "asset_redistribution": {
    "equipment": "return_to_pool",
    "licenses": "transfer_to_department",
    "budget_remainder": "return_to_department"
  }
}
```

### Role Commands

#### Create Role
**Subject**: `cim.organization.v1.command.role.org.{org_id}.create`
```json
{
  "id": "role-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "title": "Senior Software Engineer",
  "department_id": "dept-uuid",
  "team_id": "team-uuid",
  "role_type": "individual_contributor",
  "level": "senior",
  "reporting_manager_role": "engineering-manager-role-uuid",
  "responsibilities": [
    "Design and implement software features",
    "Mentor junior engineers",
    "Participate in architectural decisions",
    "Code review and quality assurance"
  ],
  "required_qualifications": [
    "5+ years software development experience",
    "Bachelor's degree in Computer Science or equivalent",
    "Experience with distributed systems",
    "Strong communication skills"
  ],
  "compensation_range": {
    "min_salary": 140000,
    "max_salary": 180000,
    "currency": "USD",
    "equity_percentage": 0.05
  },
  "benefits": [
    "health_insurance",
    "dental_insurance",
    "401k_matching",
    "unlimited_pto"
  ]
}
```

#### Assign Role
**Subject**: `cim.organization.v1.command.role.org.{org_id}.assign`
```json
{
  "role_id": "role-uuid",
  "person_id": "person-uuid",
  "assignment_type": "permanent",
  "start_date": "2025-01-15",
  "probation_period": "3_months",
  "compensation": {
    "base_salary": 165000,
    "currency": "USD",
    "equity_grant": 1000,
    "signing_bonus": 25000
  },
  "reporting_structure": {
    "direct_manager": "manager-person-uuid",
    "skip_level_manager": "director-person-uuid"
  },
  "onboarding_plan": {
    "buddy_assigned": "buddy-person-uuid",
    "training_modules": [
      "company_orientation",
      "engineering_practices",
      "security_training"
    ],
    "equipment_allocation": "standard_engineering_setup"
  },
  "assigned_by": "hr-system"
}
```

### Policy Commands

#### Create Policy
**Subject**: `cim.organization.v1.command.policy.org.{org_id}.create`
```json
{
  "id": "policy-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "policy_name": "Code of Conduct Policy",
  "policy_type": "behavioral",
  "category": "workplace_conduct",
  "version": "1.0",
  "effective_date": "2025-01-01T00:00:00Z",
  "scope": {
    "applies_to": "all_employees",
    "locations": "all_locations",
    "contractors_included": true
  },
  "policy_content": {
    "summary": "Guidelines for professional conduct and workplace behavior",
    "detailed_policy_url": "https://policies.company.com/code-of-conduct-v1.0",
    "key_requirements": [
      "Treat all colleagues with respect",
      "Maintain confidentiality of sensitive information",
      "Report violations through proper channels",
      "Comply with all applicable laws and regulations"
    ]
  },
  "enforcement": {
    "violation_reporting": "anonymous_hotline",
    "investigation_process": "hr_led",
    "disciplinary_actions": [
      "verbal_warning",
      "written_warning",
      "suspension",
      "termination"
    ]
  },
  "training_requirement": {
    "required": true,
    "frequency": "annual",
    "completion_deadline": "30_days_from_hire"
  },
  "review_schedule": {
    "frequency": "annual",
    "next_review_date": "2025-12-31",
    "responsible_team": "hr_legal_team"
  }
}
```

#### Update Policy
**Subject**: `cim.organization.v1.command.policy.org.{org_id}.update`
```json
{
  "policy_id": "policy-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "version": "1.1",
  "changes": {
    "change_type": "amendment",
    "change_summary": "Added remote work guidelines",
    "modified_sections": [
      "workplace_behavior",
      "communication_standards"
    ]
  },
  "effective_date": "2025-02-01T00:00:00Z",
  "notification_plan": {
    "all_employees_notification": true,
    "training_update_required": true,
    "acknowledgment_required": true,
    "deadline": "2025-02-15"
  },
  "approved_by": "legal-team-lead",
  "legal_review_completed": true
}
```

### Resource Commands

#### Allocate Resource
**Subject**: `cim.organization.v1.command.resource.org.{org_id}.allocate`
```json
{
  "id": "resource-allocation-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "resource_type": "budget",
  "allocation_target": {
    "target_type": "department",
    "target_id": "dept-uuid"
  },
  "resource_details": {
    "amount": 2500000,
    "currency": "USD",
    "allocation_period": "2025-q1",
    "purpose": "operational_expenses"
  },
  "allocation_breakdown": {
    "personnel": 60,
    "technology": 25,
    "marketing": 10,
    "operations": 5
  },
  "constraints": {
    "spending_approval_threshold": 50000,
    "reallocation_requires_approval": true,
    "reporting_frequency": "monthly"
  },
  "allocated_by": "cfo-user-id",
  "allocation_date": "2025-01-01T00:00:00Z"
}
```

#### Request Resource
**Subject**: `cim.organization.v1.command.resource.dept.{dept_id}.request`
```json
{
  "id": "resource-request-uuid",
  "requesting_department": "dept-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "resource_type": "additional_headcount",
  "request_details": {
    "headcount_increase": 5,
    "roles_requested": [
      {
        "title": "Senior Software Engineer",
        "count": 3,
        "urgency": "high"
      },
      {
        "title": "Product Manager",
        "count": 2,
        "urgency": "medium"
      }
    ]
  },
  "business_justification": {
    "reason": "increased_product_demand",
    "expected_roi": "300%",
    "timeline": "Q2_2025",
    "risk_if_not_approved": "missed_revenue_targets"
  },
  "budget_impact": {
    "annual_cost": 850000,
    "currency": "USD",
    "funding_source": "growth_budget"
  },
  "requested_by": "engineering-director-uuid",
  "priority": "high",
  "deadline": "2025-03-01"
}
```

## Event Messages

Events are published to NATS subjects following the same envelope pattern as commands.

### Organization Events

#### Organization Created
**Subject**: `cim.organization.v1.event.organization.created`
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "name": "Acme Corporation",
  "legal_name": "Acme Corporation Inc.",
  "organization_type": "corporation",
  "industry": "technology",
  "size": "medium",
  "headquarters": {
    "city": "San Francisco",
    "state": "CA",
    "country": "US"
  },
  "founding_date": "2020-01-15",
  "created_at": "2025-01-15T10:30:00Z",
  "created_by": "system"
}
```

#### Organization Merged
**Subject**: `cim.organization.v1.event.organization.merged`
```json
{
  "parent_organization_id": "parent-org-id",
  "acquired_organization_id": "acquired-org-id",
  "merger_type": "acquisition",
  "effective_date": "2025-03-01T00:00:00Z",
  "transaction_value": {
    "amount": 50000000,
    "currency": "USD"
  },
  "integration_status": "planning_phase",
  "merged_at": "2025-01-15T14:30:00Z",
  "approved_by": "board-of-directors"
}
```

### Department Events

#### Department Created
**Subject**: `cim.organization.v1.event.department.org.{org_id}.created`
```json
{
  "department_id": "dept-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "name": "Engineering",
  "department_type": "operational",
  "head_of_department": "manager-uuid",
  "budget_allocated": {
    "amount": 5000000,
    "currency": "USD"
  },
  "location": "San Francisco HQ",
  "created_at": "2025-01-15T10:30:00Z",
  "created_by": "hr-system"
}
```

#### Department Restructured
**Subject**: `cim.organization.v1.event.department.org.{org_id}.restructured`
```json
{
  "department_id": "dept-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "restructure_type": "split",
  "previous_structure": {
    "teams": ["unified_engineering_team"],
    "team_count": 1,
    "employee_count": 45
  },
  "new_structure": {
    "departments": [
      {
        "name": "Frontend Engineering",
        "team_count": 3,
        "employee_count": 18
      },
      {
        "name": "Backend Engineering", 
        "team_count": 3,
        "employee_count": 27
      }
    ]
  },
  "effective_date": "2025-04-01T00:00:00Z",
  "restructured_at": "2025-01-15T11:15:00Z",
  "restructured_by": "cto-user-id"
}
```

### Team Events

#### Team Formed
**Subject**: `cim.organization.v1.event.team.dept.{dept_id}.formed`
```json
{
  "team_id": "team-uuid",
  "department_id": "dept-uuid",
  "name": "Platform Engineering Team",
  "team_type": "permanent",
  "team_lead": "lead-uuid",
  "initial_member_count": 2,
  "target_size": 8,
  "formation_date": "2025-01-15",
  "budget_allocated": {
    "amount": 1200000,
    "currency": "USD"
  },
  "formed_at": "2025-01-15T09:00:00Z",
  "formed_by": "engineering-director"
}
```

### Role Events

#### Role Created
**Subject**: `cim.organization.v1.event.role.org.{org_id}.created`
```json
{
  "role_id": "role-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "title": "Senior Software Engineer",
  "department_id": "dept-uuid",
  "team_id": "team-uuid",
  "level": "senior",
  "compensation_range": {
    "min_salary": 140000,
    "max_salary": 180000,
    "currency": "USD"
  },
  "created_at": "2025-01-15T10:00:00Z",
  "created_by": "hr-manager"
}
```

#### Role Assigned
**Subject**: `cim.organization.v1.event.role.org.{org_id}.assigned`
```json
{
  "role_id": "role-uuid",
  "person_id": "person-uuid",
  "assignment_type": "permanent",
  "start_date": "2025-01-15",
  "compensation": {
    "base_salary": 165000,
    "currency": "USD",
    "equity_grant": 1000
  },
  "reporting_manager": "manager-person-uuid",
  "assigned_at": "2025-01-10T16:30:00Z",
  "assigned_by": "hr-system"
}
```

### Policy Events

#### Policy Created
**Subject**: `cim.organization.v1.event.policy.org.{org_id}.created`
```json
{
  "policy_id": "policy-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "policy_name": "Code of Conduct Policy",
  "policy_type": "behavioral",
  "version": "1.0",
  "effective_date": "2025-01-01T00:00:00Z",
  "scope": "all_employees",
  "training_required": true,
  "created_at": "2025-01-15T08:00:00Z",
  "created_by": "legal-team"
}
```

#### Policy Violation Detected
**Subject**: `cim.organization.v1.event.policy.org.{org_id}.violation_detected`
```json
{
  "violation_id": "violation-uuid",
  "policy_id": "policy-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "violation_type": "code_of_conduct",
  "severity": "moderate",
  "reported_by": "anonymous",
  "involved_parties": [
    {
      "person_id": "person-1-uuid",
      "role": "accused"
    },
    {
      "person_id": "person-2-uuid", 
      "role": "witness"
    }
  ],
  "incident_details": {
    "date": "2025-01-10",
    "location": "office_meeting_room_a",
    "description": "Inappropriate comments during team meeting"
  },
  "investigation_status": "initiated",
  "detected_at": "2025-01-15T13:45:00Z"
}
```

### Resource Events

#### Resource Allocated
**Subject**: `cim.organization.v1.event.resource.org.{org_id}.allocated`
```json
{
  "allocation_id": "resource-allocation-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "resource_type": "budget",
  "target_department": "dept-uuid",
  "amount": {
    "value": 2500000,
    "currency": "USD"
  },
  "allocation_period": "2025-q1",
  "purpose": "operational_expenses",
  "allocated_at": "2025-01-01T00:00:00Z",
  "allocated_by": "cfo-user-id"
}
```

## Query Messages

Query messages use request-reply pattern over NATS.

### Get Organization
**Subject**: `cim.organization.v1.query.organization.get`
**Request**:
```json
{
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef"
}
```
**Response**:
```json
{
  "id": "01234567-89ab-cdef-0123-456789abcdef",
  "name": "Acme Corporation",
  "legal_name": "Acme Corporation Inc.",
  "organization_type": "corporation",
  "industry": "technology",
  "size": "medium",
  "employee_count": 245,
  "headquarters": {
    "address": "123 Tech Street",
    "city": "San Francisco",
    "state": "CA",
    "country": "US"
  },
  "founded": "2020-01-15",
  "financial_summary": {
    "annual_revenue": 50000000,
    "annual_budget": 35000000,
    "currency": "USD"
  },
  "departments": [
    {
      "id": "dept-1-uuid",
      "name": "Engineering",
      "employee_count": 85
    },
    {
      "id": "dept-2-uuid", 
      "name": "Sales",
      "employee_count": 45
    }
  ],
  "status": "active",
  "created_at": "2025-01-15T10:30:00Z",
  "updated_at": "2025-01-15T10:35:00Z"
}
```

### Search Organizations
**Subject**: `cim.organization.v1.query.organization.search`
**Request**:
```json
{
  "criteria": {
    "industry": "technology",
    "size": ["medium", "large"],
    "location": {
      "country": "US",
      "state": "CA"
    },
    "status": "active"
  },
  "limit": 50,
  "offset": 0,
  "sort": {
    "field": "employee_count",
    "direction": "desc"
  }
}
```

### Get Department Hierarchy
**Subject**: `cim.organization.v1.query.department.hierarchy`
**Request**:
```json
{
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "include_teams": true,
  "include_roles": false,
  "depth_limit": 3
}
```

### Get Role Details
**Subject**: `cim.organization.v1.query.role.get`
**Request**:
```json
{
  "role_id": "role-uuid",
  "include_requirements": true,
  "include_compensation": true,
  "include_current_assignment": true
}
```

### Search Policies
**Subject**: `cim.organization.v1.query.policy.search`
**Request**:
```json
{
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "policy_type": "behavioral",
  "status": "active",
  "effective_date_range": {
    "start": "2025-01-01",
    "end": "2025-12-31"
  }
}
```

## Workflow Messages

Workflow orchestration for complex organizational operations.

### Organization Onboarding Workflow
**Subject**: `cim.organization.v1.workflow.onboarding.start`
```json
{
  "workflow_id": "onboarding-workflow-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "workflow_type": "organization_onboarding",
  "steps": [
    {
      "step_id": "legal_setup",
      "description": "Complete legal entity setup",
      "estimated_duration": "5_days",
      "dependencies": []
    },
    {
      "step_id": "initial_structure",
      "description": "Create initial organizational structure",
      "estimated_duration": "3_days", 
      "dependencies": ["legal_setup"]
    },
    {
      "step_id": "policy_framework",
      "description": "Establish core policies",
      "estimated_duration": "7_days",
      "dependencies": ["legal_setup"]
    },
    {
      "step_id": "system_setup",
      "description": "Configure organizational systems",
      "estimated_duration": "10_days",
      "dependencies": ["initial_structure"]
    }
  ],
  "priority": "high",
  "initiated_by": "setup-wizard",
  "target_completion": "2025-02-15"
}
```

### Department Restructuring Workflow
**Subject**: `cim.organization.v1.workflow.restructuring.start`
```json
{
  "workflow_id": "restructuring-workflow-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "department_id": "dept-uuid",
  "restructuring_type": "merger",
  "source_departments": [
    "frontend-dept-uuid",
    "backend-dept-uuid"
  ],
  "target_structure": {
    "new_department": {
      "name": "Full-Stack Engineering",
      "expected_team_count": 6,
      "expected_employee_count": 72
    }
  },
  "timeline": {
    "planning_phase": "2_weeks",
    "communication_phase": "1_week",
    "implementation_phase": "4_weeks",
    "stabilization_phase": "2_weeks"
  },
  "stakeholders": [
    "cto-user-id",
    "hr-director-id",
    "affected-managers"
  ],
  "impact_assessment": {
    "employee_impact": "medium",
    "system_changes": "high",
    "budget_reallocation": true
  }
}
```

## Analytics Messages

Organizational performance measurement and analysis.

### Performance Measurement
**Subject**: `cim.organization.v1.analytics.performance.measure`
```json
{
  "measurement_id": "measurement-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "scope": {
    "scope_type": "department",
    "scope_id": "dept-uuid"
  },
  "metrics": [
    {
      "metric_name": "employee_satisfaction",
      "value": 4.2,
      "scale": "1_to_5",
      "measurement_date": "2025-01-15"
    },
    {
      "metric_name": "productivity_index",
      "value": 87.5,
      "scale": "0_to_100",
      "measurement_date": "2025-01-15"
    },
    {
      "metric_name": "retention_rate",
      "value": 94.2,
      "scale": "percentage",
      "measurement_date": "2025-01-15"
    }
  ],
  "reporting_period": "2024-q4",
  "benchmarks": {
    "industry_average": 82.1,
    "company_target": 90.0,
    "previous_period": 85.3
  }
}
```

### Organizational Health Assessment
**Subject**: `cim.organization.v1.analytics.health.assess`
```json
{
  "assessment_id": "health-assessment-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "assessment_type": "comprehensive",
  "dimensions": {
    "financial_health": {
      "score": 8.5,
      "trend": "improving",
      "key_indicators": ["revenue_growth", "profit_margin", "cash_flow"]
    },
    "operational_efficiency": {
      "score": 7.8,
      "trend": "stable",
      "key_indicators": ["process_automation", "resource_utilization", "cycle_time"]
    },
    "employee_engagement": {
      "score": 8.2,
      "trend": "improving",
      "key_indicators": ["satisfaction", "retention", "productivity"]
    },
    "innovation_capacity": {
      "score": 7.5,
      "trend": "stable",
      "key_indicators": ["r_and_d_investment", "patent_applications", "new_product_launches"]
    }
  },
  "overall_health_score": 8.0,
  "assessment_date": "2025-01-15",
  "recommendations": [
    "Increase investment in innovation programs",
    "Implement additional process automation",
    "Enhance employee development programs"
  ]
}
```

## Compliance Messages

Compliance monitoring and audit operations.

### Compliance Audit
**Subject**: `cim.organization.v1.compliance.audit.initiate`
```json
{
  "audit_id": "audit-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "audit_type": "internal",
  "compliance_framework": "sox",
  "scope": {
    "departments": ["finance", "accounting", "it"],
    "processes": ["financial_reporting", "access_controls", "data_management"],
    "time_period": {
      "start": "2024-01-01",
      "end": "2024-12-31"
    }
  },
  "auditor": {
    "lead_auditor": "external-auditor-id",
    "audit_firm": "BigFour Auditing LLC",
    "audit_team_size": 5
  },
  "timeline": {
    "planning_phase": "2_weeks",
    "fieldwork_phase": "6_weeks", 
    "reporting_phase": "2_weeks",
    "expected_completion": "2025-03-15"
  },
  "deliverables": [
    "management_letter",
    "compliance_report",
    "remediation_recommendations",
    "control_effectiveness_assessment"
  ]
}
```

### Compliance Violation Report
**Subject**: `cim.organization.v1.compliance.violation.report`
```json
{
  "violation_id": "violation-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "violation_type": "data_privacy",
  "regulatory_framework": "gdpr",
  "severity": "high",
  "description": "Unauthorized access to customer personal data",
  "affected_systems": ["customer_database", "crm_system"],
  "data_subjects_affected": 1250,
  "incident_details": {
    "discovery_date": "2025-01-10",
    "incident_date": "2025-01-08",
    "discovery_method": "security_monitoring",
    "root_cause": "misconfigured_access_permissions"
  },
  "immediate_actions": [
    "Revoked unauthorized access",
    "Secured affected systems",
    "Initiated forensic investigation",
    "Notified data protection officer"
  ],
  "regulatory_reporting": {
    "notification_required": true,
    "notification_deadline": "2025-01-12",
    "regulatory_body": "data_protection_authority"
  },
  "remediation_plan": {
    "technical_fixes": "access_control_audit_and_update",
    "process_improvements": "enhanced_monitoring_procedures", 
    "training_requirements": "security_awareness_refresher",
    "completion_target": "2025-01-31"
  }
}
```

## Message Patterns

### Request-Reply Pattern
For queries and synchronous operations:

```javascript
// JavaScript example using NATS.js
const msg = await nc.request(
  'cim.organization.v1.query.organization.get',
  JSON.stringify({
    correlation_id: uuidv4(),
    causation_id: uuidv4(),
    message_id: uuidv4(),
    timestamp: new Date().toISOString(),
    version: '1.0',
    payload: { organization_id: orgId }
  }),
  { timeout: 30000 }
);
```

### Publish-Subscribe Pattern
For events and async operations:

```javascript
// Publish a command
await nc.publish(
  'cim.organization.v1.command.organization.create',
  JSON.stringify({
    correlation_id: uuidv4(),
    causation_id: uuidv4(),
    message_id: uuidv4(),
    timestamp: new Date().toISOString(),
    version: '1.0',
    payload: {
      id: orgId,
      name: 'New Organization',
      legal_name: 'New Organization Inc.',
      organization_type: 'corporation'
    }
  })
);

// Subscribe to events
const sub = nc.subscribe('cim.organization.v1.event.organization.created');
for await (const msg of sub) {
  const envelope = JSON.parse(msg.data);
  console.log('Organization created:', envelope.payload);
}
```

### Workflow Processing Pattern
For complex organizational workflows:

```javascript
// Subscribe to workflow events with queue groups
const sub = nc.subscribe(
  'cim.organization.v1.workflow.onboarding.*',
  { queue: 'onboarding-processors' }
);

for await (const msg of sub) {
  const envelope = JSON.parse(msg.data);
  
  switch (msg.subject) {
    case 'cim.organization.v1.workflow.onboarding.start':
      await initiateOnboardingWorkflow(envelope.payload);
      break;
    case 'cim.organization.v1.workflow.onboarding.step_completed':
      await processOnboardingStep(envelope.payload);
      break;
    case 'cim.organization.v1.workflow.onboarding.completed':
      await finalizeOnboarding(envelope.payload);
      break;
  }
}
```

## Error Handling

### Error Events

Errors are communicated through NATS error events.

#### Organization Creation Failed
**Subject**: `cim.organization.v1.event.organization.creation_failed`
```json
{
  "original_command_id": "command-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "error": {
    "code": "LEGAL_VALIDATION_FAILED",
    "message": "Organization name already exists in jurisdiction",
    "category": "validation",
    "retry_possible": false,
    "details": {
      "existing_organization": "existing-org-id",
      "jurisdiction": "Delaware",
      "suggested_names": [
        "Acme Corporation II",
        "Acme Technologies Corp",
        "New Acme Corporation"
      ]
    }
  },
  "failed_at": "2025-01-15T10:35:00Z"
}
```

#### Policy Enforcement Failed
**Subject**: `cim.organization.v1.event.policy.enforcement_failed`
```json
{
  "enforcement_id": "enforcement-uuid",
  "policy_id": "policy-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "error": {
    "code": "INSUFFICIENT_AUTHORITY",
    "message": "User lacks authority to enforce policy violation consequences",
    "category": "authorization",
    "required_permissions": ["policy_enforcement", "disciplinary_action"],
    "current_permissions": ["policy_view", "violation_reporting"]
  },
  "escalation": {
    "escalated_to": "hr-director-id",
    "escalation_reason": "authority_required",
    "escalation_timestamp": "2025-01-15T11:00:00Z"
  },
  "failed_at": "2025-01-15T10:45:00Z"
}
```

#### Budget Allocation Failed
**Subject**: `cim.organization.v1.event.resource.allocation_failed`
```json
{
  "allocation_id": "allocation-uuid",
  "organization_id": "01234567-89ab-cdef-0123-456789abcdef",
  "error": {
    "code": "INSUFFICIENT_FUNDS",
    "message": "Requested allocation exceeds available budget",
    "category": "business_rule",
    "details": {
      "requested_amount": 5000000,
      "available_amount": 3500000,
      "currency": "USD",
      "budget_period": "2025-q1"
    }
  },
  "suggested_actions": [
    "Request budget increase approval",
    "Reduce allocation amount",
    "Defer allocation to next period",
    "Reallocate from other departments"
  ],
  "failed_at": "2025-01-15T09:30:00Z"
}
```

### Error Codes

Standard error codes used across the Organization Domain:

- `VALIDATION_ERROR` - Input validation failed
- `LEGAL_VALIDATION_FAILED` - Legal requirements not met
- `NOT_FOUND` - Requested organization/resource not found
- `PERMISSION_DENIED` - Insufficient permissions
- `INSUFFICIENT_FUNDS` - Budget allocation exceeds available funds
- `DUPLICATE_ENTITY` - Organization/department already exists
- `HIERARCHY_VIOLATION` - Organizational structure constraint violated
- `POLICY_VIOLATION` - Business policy constraint violated
- `REGULATORY_COMPLIANCE_ERROR` - Regulatory requirement violation
- `WORKFLOW_STATE_ERROR` - Invalid workflow state transition
- `RESOURCE_CONFLICT` - Resource allocation conflict
- `APPROVAL_REQUIRED` - Operation requires higher-level approval

### Recovery Patterns

Services implement automatic recovery through NATS:

1. **Retry with Backoff**: Resubmit commands with exponential delay
2. **Approval Escalation**: Route failed operations requiring approval
3. **Workflow Compensation**: Rollback complex multi-step operations
4. **Dead Letter Queue**: Failed messages to `cim.organization.v1.dlq.*`

## Subject Algebra

The Organization Domain implements a formal Subject Algebra that provides mathematical foundations for organizational operations. See [Organization Subject Algebra](../algebra/README.md) for complete mathematical definitions.

### Subject Structure
```
organization.{domain}.{version}.{operation}.{aggregate}.{scope}.{qualifier}
```

**Formal Grammar**:
```bnf
<org-subject> ::= "cim.organization." <version> "." <operation> "." <aggregate> "." <scope> ["." <qualifier>]

<version>    ::= "v1" | "v2" | "v3"
<operation>  ::= "command" | "event" | "query" | "workflow" | "analytics" | "compliance" | "integration"
<aggregate>  ::= "organization" | "department" | "team" | "role" | "policy" | "resource" | "structure" | "culture" | "strategy" | "performance" | "communication" | "change" | "risk" | "vendor" | "location"
<scope>      ::= <global-scope> | <org-scope> | <dept-scope> | <team-scope> | <role-scope> | <location-scope> | <region-scope> | <division-scope> | <project-scope> | <cost-center-scope> | <vendor-scope>
```

### Core Subjects

#### Organization Lifecycle Operations
```
# Basic organization lifecycle
cim.organization.v1.command.organization.create
cim.organization.v1.command.organization.update
cim.organization.v1.command.organization.merge
cim.organization.v1.command.organization.dissolve

# With qualifiers
cim.organization.v1.command.organization.create.type.corporation
cim.organization.v1.command.organization.update.scope.headquarters_relocation
cim.organization.v1.command.organization.merge.method.acquisition
```

#### Department Management
```
# Department operations
cim.organization.v1.command.department.org.{org_id}.create.type.operational
cim.organization.v1.command.department.org.{org_id}.restructure.method.split
cim.organization.v1.command.department.org.{org_id}.merge.target.{target_dept_id}

# Department events
cim.organization.v1.event.department.org.{org_id}.created.budget.allocated
cim.organization.v1.event.department.org.{org_id}.performance.measured.kpi.exceeded
```

#### Resource Management
```
# Resource allocation
cim.organization.v1.command.resource.org.{org_id}.allocate.type.budget
cim.organization.v1.command.resource.dept.{dept_id}.request.type.headcount
cim.organization.v1.command.resource.team.{team_id}.provision.type.equipment

# Resource queries
cim.organization.v1.query.resource.org.{org_id}.utilization.period.monthly
cim.organization.v1.query.resource.dept.{dept_id}.availability.type.budget
```

### Algebraic Compositions

Sequential processing (⊕):
```
cim.organization.v1.event.organization.created
  → cim.organization.v1.command.department.org.{org_id}.create
  → cim.organization.v1.event.department.org.{org_id}.created
  → cim.organization.v1.command.role.org.{org_id}.create
```

Parallel processing (⊗):
```
cim.organization.v1.event.organization.created
  → cim.organization.v1.command.policy.org.{org_id}.create_defaults
  → cim.organization.v1.command.resource.org.{org_id}.allocate_initial
  → cim.organization.v1.command.compliance.org.{org_id}.initialize_framework
```

Conditional transformation (→[P]):
```
cim.organization.v1.event.resource.org.{org_id}.request_received
  → cim.organization.v1.query.resource.org.{org_id}.check_availability
  → cim.organization.v1.command.resource.org.{org_id}.approve
      [if budget_available AND authorized]
```

### Wildcard Subscriptions

For service implementations:

```javascript
// Process all organization commands
nc.subscribe('cim.organization.v1.command.*', { queue: 'org-commands' })

// Listen to all department events
nc.subscribe('cim.organization.v1.event.department.*')

// Handle all resource operations
nc.subscribe('cim.organization.v1.*.resource.*', { queue: 'resource-service' })

// Monitor all compliance events
nc.subscribe('cim.organization.v1.compliance.*')
```

## Integration Examples

### Basic Publisher (Client)
```javascript
import { connect } from 'nats';

const nc = await connect({ servers: 'nats://localhost:4222' });

// Create organization
await nc.publish(
  'cim.organization.v1.command.organization.create',
  JSON.stringify({
    correlation_id: crypto.randomUUID(),
    causation_id: crypto.randomUUID(),
    message_id: crypto.randomUUID(),
    timestamp: new Date().toISOString(),
    version: '1.0',
    payload: {
      id: crypto.randomUUID(),
      name: 'New Tech Startup',
      legal_name: 'New Tech Startup Inc.',
      organization_type: 'corporation',
      industry: 'technology'
    }
  })
);
```

### Event Processor (Service)
```javascript
// Subscribe to organization creation events
const sub = nc.subscribe('cim.organization.v1.event.organization.created');
for await (const msg of sub) {
  const envelope = JSON.parse(msg.data);
  
  // Start organization setup workflow
  await nc.publish(
    'cim.organization.v1.workflow.onboarding.start',
    JSON.stringify({
      correlation_id: envelope.correlation_id,
      causation_id: envelope.message_id,
      message_id: crypto.randomUUID(),
      timestamp: new Date().toISOString(),
      version: '1.0',
      payload: {
        organization_id: envelope.payload.id,
        workflow_type: 'organization_onboarding',
        priority: 'high'
      }
    })
  );
}
```

### Compliance Monitoring Handler
```javascript
// Handle compliance audit workflows
const sub = nc.subscribe('cim.organization.v1.compliance.audit.*');
for await (const msg of sub) {
  const envelope = JSON.parse(msg.data);
  
  try {
    switch (msg.subject) {
      case 'cim.organization.v1.compliance.audit.initiate':
        const auditPlan = await createAuditPlan(envelope.payload);
        
        await nc.publish(
          'cim.organization.v1.event.compliance.audit.planned',
          JSON.stringify({
            correlation_id: envelope.correlation_id,
            causation_id: envelope.message_id,
            message_id: crypto.randomUUID(),
            timestamp: new Date().toISOString(),
            version: '1.0',
            payload: {
              audit_id: envelope.payload.audit_id,
              organization_id: envelope.payload.organization_id,
              audit_plan: auditPlan,
              scheduled_start: auditPlan.start_date
            }
          })
        );
        break;
        
      case 'cim.organization.v1.compliance.violation.report':
        await processComplianceViolation(envelope.payload);
        break;
    }
  } catch (error) {
    // Publish compliance processing error
    await nc.publish(
      'cim.organization.v1.event.compliance.processing_failed',
      JSON.stringify({
        correlation_id: envelope.correlation_id,
        causation_id: envelope.message_id,
        message_id: crypto.randomUUID(),
        timestamp: new Date().toISOString(),
        version: '1.0',
        payload: {
          original_subject: msg.subject,
          error: {
            code: 'COMPLIANCE_PROCESSING_ERROR',
            message: error.message,
            category: 'system_error'
          }
        }
      })
    );
  }
}
```