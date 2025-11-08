//! Integration tests for the Organization domain
//!
//! ## Test Mermaid Diagram
//!
//! ```mermaid
//! graph TD
//!     A[Create Organization] --> B[Create Departments]
//!     B --> C[Create Teams]
//!     C --> D[Create Roles/Positions]
//!     D --> E[Create Facilities/Places]
//!     E --> F[Test Hierarchy]
//!     F --> G[Test Status Transitions]
//!     G --> H[Test Dissolution/Merger]
//! ```
//!
//! ## Domain Purity
//!
//! These tests verify that the Organization domain maintains pure boundaries:
//! - Tests POSITIONS (roles) not PEOPLE
//! - Tests PLACES (facilities) not LOCATIONS
//! - No person-role assignments (that's Association domain)
//! - No facility-location links (that's Association domain)

use cim_domain_organization::*;
use uuid::Uuid;

#[test]
fn test_create_organization_complete_flow() {
    // Create an empty aggregate for testing CreateOrganization command
    let mut org = OrganizationAggregate::empty();

    // Verify initial state
    assert_eq!(org.status, OrganizationStatus::Pending);
    assert!(org.departments.is_empty());
    assert!(org.teams.is_empty());
    assert!(org.roles.is_empty());
    assert!(org.facilities.is_empty());

    // Create organization command
    let message_id = Uuid::now_v7();
    let create_cmd = CreateOrganization {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        name: "Acme Corporation".to_string(),
        display_name: "Acme Corporation".to_string(),
        description: Some("A test corporation".to_string()),
        organization_type: OrganizationType::Corporation,
        parent_id: None,
        founded_date: None,
        metadata: serde_json::json!({}),
    };

    let events = org
        .handle_command(OrganizationCommand::CreateOrganization(create_cmd))
        .unwrap();
    assert_eq!(events.len(), 1);

    // Apply event
    org.apply_event(&events[0]).unwrap();
    assert_eq!(org.status, OrganizationStatus::Active);
}

#[test]
fn test_department_management() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Tech Company".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Create Engineering Department
    let message_id = Uuid::now_v7();
    let create_dept_cmd = CreateDepartment {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: EntityId::from_uuid(org_id),
        parent_department_id: None,
        name: "Engineering".to_string(),
        code: "ENG".to_string(),
        description: Some("Engineering department".to_string()),
    };

    let events = org
        .handle_command(OrganizationCommand::CreateDepartment(create_dept_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.departments.len(), 1);
}

#[test]
fn test_team_management() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Team Test Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Create a team
    let message_id = Uuid::now_v7();
    let create_team_cmd = CreateTeam {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: EntityId::from_uuid(org_id),
        department_id: None,
        name: "Backend Team".to_string(),
        description: Some("Backend development team".to_string()),
        team_type: TeamType::Permanent,
        max_members: Some(10),
    };

    let events = org
        .handle_command(OrganizationCommand::CreateTeam(create_team_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.teams.len(), 1);
}

#[test]
fn test_role_management() {
    // Test creating organizational POSITIONS (not assigning people to them)
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Role Test Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Create CEO position
    let message_id = Uuid::now_v7();
    let create_ceo_cmd = CreateRole {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: EntityId::from_uuid(org_id),
        department_id: None,
        team_id: None,
        title: "Chief Executive Officer".to_string(),
        code: "CEO".to_string(),
        description: Some("Top executive position".to_string()),
        role_type: RoleType::Executive,
        level: Some(10),
        reports_to: None,
        permissions: vec!["all".to_string()],
        responsibilities: vec!["Strategic direction".to_string()],
    };

    let events = org
        .handle_command(OrganizationCommand::CreateRole(create_ceo_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.roles.len(), 1);

    // Get the CEO role ID
    let ceo_role_id = org.roles.keys().next().unwrap().clone();

    // Create CTO position that reports to CEO
    let message_id2 = Uuid::now_v7();
    let create_cto_cmd = CreateRole {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        organization_id: EntityId::from_uuid(org_id),
        department_id: None,
        team_id: None,
        title: "Chief Technology Officer".to_string(),
        code: "CTO".to_string(),
        description: Some("Technology leader".to_string()),
        role_type: RoleType::Executive,
        level: Some(9),
        reports_to: Some(ceo_role_id.clone()),  // ROLE-TO-ROLE hierarchy
        permissions: vec!["tech_decisions".to_string()],
        responsibilities: vec!["Technology strategy".to_string()],
    };

    let events = org
        .handle_command(OrganizationCommand::CreateRole(create_cto_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.roles.len(), 2);

    // Verify the reporting structure is role-to-role
    let cto_role = org.roles.values().find(|r| r.code == "CTO").unwrap();
    assert_eq!(cto_role.reports_to, Some(ceo_role_id));
}

#[test]
fn test_facility_management() {
    // Test creating organizational PLACES (not linking them to addresses)
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Facility Test Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Create headquarters facility (NO address data)
    let message_id = Uuid::now_v7();
    let create_hq_cmd = CreateFacility {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: EntityId::from_uuid(org_id),
        name: "Headquarters".to_string(),
        code: "HQ-001".to_string(),
        facility_type: FacilityType::Headquarters,
        description: Some("Main office building".to_string()),
        capacity: Some(500),
        parent_facility_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::CreateFacility(create_hq_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.facilities.len(), 1);

    // Create warehouse facility
    let message_id2 = Uuid::now_v7();
    let create_warehouse_cmd = CreateFacility {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        organization_id: EntityId::from_uuid(org_id),
        name: "Main Warehouse".to_string(),
        code: "WH-001".to_string(),
        facility_type: FacilityType::Warehouse,
        description: Some("Primary storage facility".to_string()),
        capacity: Some(10000),
        parent_facility_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::CreateFacility(create_warehouse_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.facilities.len(), 2);

    // Verify facilities have NO address data
    let hq = org.facilities.values().find(|f| f.code == "HQ-001").unwrap();
    assert_eq!(hq.facility_type, FacilityType::Headquarters);
    assert_eq!(hq.capacity, Some(500));
    // Note: NO address field exists in Facility struct
}

#[test]
fn test_organization_hierarchy() {
    // Create parent company
    let company_id = Uuid::now_v7();
    let mut company = OrganizationAggregate::new(
        company_id,
        "Global Corp".to_string(),
        OrganizationType::Corporation,
    );
    company.status = OrganizationStatus::Active;

    // Create division
    let division_id = Uuid::now_v7();
    let message_id = Uuid::now_v7();
    let add_division_cmd = AddChildOrganization {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        parent_organization_id: company_id,
        child_organization_id: division_id,
        child_name: "Tech Division".to_string(),
        child_type: OrganizationType::Corporation,
    };

    let events = company
        .handle_command(OrganizationCommand::AddChildOrganization(add_division_cmd))
        .unwrap();
    company.apply_event(&events[0]).unwrap();

    assert_eq!(company.child_organizations.len(), 1);

    // Test self-reference prevention
    let message_id2 = Uuid::now_v7();
    let self_ref_cmd = AddChildOrganization {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        parent_organization_id: company_id,
        child_organization_id: company_id,
        child_name: "Self Reference".to_string(),
        child_type: OrganizationType::Corporation,
    };

    let result = company.handle_command(OrganizationCommand::AddChildOrganization(self_ref_cmd));
    assert!(result.is_err());
}

#[test]
fn test_organization_status_transitions() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Status Test Corp".to_string(),
        OrganizationType::Corporation,
    );

    // Initial status is Pending
    assert_eq!(org.status, OrganizationStatus::Pending);

    // Activate organization
    org.status = OrganizationStatus::Active;

    // Test valid transition: Active -> Inactive
    let message_id = Uuid::now_v7();
    let inactive_cmd = ChangeOrganizationStatus {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: org_id,
        new_status: OrganizationStatus::Inactive,
        reason: Some("Temporary closure".to_string()),
    };

    let events = org
        .handle_command(OrganizationCommand::ChangeOrganizationStatus(inactive_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();
    assert_eq!(org.status, OrganizationStatus::Inactive);

    // Test invalid transition: Inactive -> Merged (must go through Active)
    let message_id2 = Uuid::now_v7();
    let invalid_cmd = ChangeOrganizationStatus {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        organization_id: org_id,
        new_status: OrganizationStatus::Merged,
        reason: None,
    };

    let result = org.handle_command(OrganizationCommand::ChangeOrganizationStatus(invalid_cmd));
    assert!(result.is_err());

    // Test valid transition: Inactive -> Active
    let message_id3 = Uuid::now_v7();
    let reactivate_cmd = ChangeOrganizationStatus {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id3),
            causation_id: cim_domain::CausationId(message_id3),
            message_id: message_id3,
        },
        organization_id: org_id,
        new_status: OrganizationStatus::Active,
        reason: Some("Reopening".to_string()),
    };

    let events = org
        .handle_command(OrganizationCommand::ChangeOrganizationStatus(reactivate_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();
    assert_eq!(org.status, OrganizationStatus::Active);
}

#[test]
fn test_organization_dissolution() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "To Be Dissolved Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Dissolve organization
    let dissolve_cmd = DissolveOrganization {
        identity: {
            let id = Uuid::now_v7();
            MessageIdentity {
                correlation_id: cim_domain::CorrelationId::Single(id),
                causation_id: cim_domain::CausationId(id),
                message_id: id,
            }
        },
        organization_id: EntityId::from_uuid(org_id),
        reason: "Bankruptcy".to_string(),
        effective_date: chrono::Utc::now(),
    };

    let events = org
        .handle_command(OrganizationCommand::DissolveOrganization(dissolve_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.status, OrganizationStatus::Dissolved);
}

#[test]
fn test_organization_merger() {
    let source_id = Uuid::now_v7();
    let target_id = Uuid::now_v7();

    let mut source_org = OrganizationAggregate::new(
        source_id,
        "Small Startup".to_string(),
        OrganizationType::Corporation,
    );
    source_org.status = OrganizationStatus::Active;

    // Test merger
    let merge_cmd = MergeOrganizations {
        identity: {
            let id = Uuid::now_v7();
            MessageIdentity {
                correlation_id: cim_domain::CorrelationId::Single(id),
                causation_id: cim_domain::CausationId(id),
                message_id: id,
            }
        },
        surviving_organization_id: EntityId::from_uuid(target_id),
        merged_organization_id: EntityId::from_uuid(source_id),
        merger_type: cim_domain_organization::events::MergerType::Acquisition,
        effective_date: chrono::Utc::now(),
    };

    let events = source_org
        .handle_command(OrganizationCommand::MergeOrganizations(merge_cmd))
        .unwrap();
    source_org.apply_event(&events[0]).unwrap();

    assert_eq!(source_org.status, OrganizationStatus::Merged);

    // Test self-merge prevention
    let self_merge_cmd = MergeOrganizations {
        identity: {
            let id = Uuid::now_v7();
            MessageIdentity {
                correlation_id: cim_domain::CorrelationId::Single(id),
                causation_id: cim_domain::CausationId(id),
                message_id: id,
            }
        },
        surviving_organization_id: EntityId::from_uuid(source_id),
        merged_organization_id: EntityId::from_uuid(source_id),
        merger_type: cim_domain_organization::events::MergerType::Merger,
        effective_date: chrono::Utc::now(),
    };

    let result = source_org.handle_command(OrganizationCommand::MergeOrganizations(self_merge_cmd));
    assert!(result.is_err());
}

#[test]
fn test_role_permissions() {
    // Test that Permission enum has organizational permissions only
    // (no person-role or facility-location permissions)

    // Create a role with organizational permissions
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Permission Test Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    let message_id = Uuid::now_v7();
    let create_role_cmd = CreateRole {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: EntityId::from_uuid(org_id),
        department_id: None,
        team_id: None,
        title: "Facility Manager".to_string(),
        code: "FAC-MGR".to_string(),
        description: Some("Manages facilities".to_string()),
        role_type: RoleType::Management,
        level: Some(5),
        reports_to: None,
        permissions: vec![
            "CreateFacility".to_string(),
            "ModifyFacility".to_string(),
            "ViewOrganization".to_string(),
        ],
        responsibilities: vec!["Manage organizational facilities".to_string()],
    };

    let events = org
        .handle_command(OrganizationCommand::CreateRole(create_role_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    let facility_mgr_role = org.roles.values().next().unwrap();
    assert!(facility_mgr_role.permissions.contains(&"CreateFacility".to_string()));
    assert!(facility_mgr_role.permissions.contains(&"ModifyFacility".to_string()));

    // Verify NO person-role permissions exist
    // (AddMember, RemoveMember, UpdateMemberRole were removed)
    assert!(!facility_mgr_role.permissions.contains(&"AddMember".to_string()));
}

#[test]
fn test_organization_size_categories() {
    assert_eq!(SizeCategory::from_employee_count(5), SizeCategory::Startup);
    assert_eq!(SizeCategory::from_employee_count(25), SizeCategory::Small);
    assert_eq!(SizeCategory::from_employee_count(100), SizeCategory::Medium);
    assert_eq!(SizeCategory::from_employee_count(500), SizeCategory::Large);
    assert_eq!(
        SizeCategory::from_employee_count(2500),
        SizeCategory::Enterprise
    );
    assert_eq!(
        SizeCategory::from_employee_count(10000),
        SizeCategory::MegaCorp
    );

    let startup = SizeCategory::Startup;
    assert_eq!(startup.typical_management_layers(), 2);
    assert_eq!(startup.employee_range(), (1, Some(10)));

    let enterprise = SizeCategory::Enterprise;
    assert_eq!(enterprise.typical_management_layers(), 6);
    let (min, max) = enterprise.typical_budget_range();
    assert_eq!(min, 500.0);
    assert_eq!(max, 2000.0);
}

#[test]
fn test_facility_update() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Facility Update Test".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Create facility
    let message_id = Uuid::now_v7();
    let create_cmd = CreateFacility {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: EntityId::from_uuid(org_id),
        name: "Office Alpha".to_string(),
        code: "OFF-A".to_string(),
        facility_type: FacilityType::Office,
        description: None,
        capacity: Some(100),
        parent_facility_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::CreateFacility(create_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    let facility_id = org.facilities.keys().next().unwrap().clone();

    // Update facility
    let message_id2 = Uuid::now_v7();
    let update_cmd = UpdateFacility {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        facility_id: facility_id.clone(),
        organization_id: EntityId::from_uuid(org_id),
        name: Some("Office Alpha Renovated".to_string()),
        code: None,
        description: Some("Newly renovated office".to_string()),
        capacity: Some(150),
        status: Some(FacilityStatus::Renovating),
        parent_facility_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::UpdateFacility(update_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    let updated_facility = org.facilities.get(&facility_id).unwrap();
    assert_eq!(updated_facility.name, "Office Alpha Renovated");
    assert_eq!(updated_facility.capacity, Some(150));
    assert_eq!(updated_facility.status, FacilityStatus::Renovating);
}
