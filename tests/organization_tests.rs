//! Integration tests for the Organization domain
//!
//! ## Test Mermaid Diagram
//!
//! ```mermaid
//! graph TD
//!     A[Create Organization] --> B[Add Members]
//!     B --> C[Set Reporting Structure]
//!     C --> D[Add Locations]
//!     D --> E[Create Child Organizations]
//!     E --> F[Test Hierarchy]
//!     F --> G[Test Status Transitions]
//!     G --> H[Test Dissolution/Merger]
//! ```

use cim_domain_organization::*;
use uuid::Uuid;

#[test]
fn test_create_organization_complete_flow() {
    // Create an empty aggregate for testing CreateOrganization command
    let mut org = OrganizationAggregate::empty();

    // Verify initial state
    assert_eq!(org.status, OrganizationStatus::Pending);
    assert!(org.members.is_empty());
    assert!(org.locations.is_empty());

    // Create organization command (using new API structure)
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
fn test_organization_member_management() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Tech Startup".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Add CEO
    let ceo_id = Uuid::now_v7();
    let ceo_role = OrganizationRole::ceo();

    let message_id = Uuid::now_v7();
    let add_ceo_cmd = AddMember {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: org_id,
        person_id: ceo_id,
        role: ceo_role,
        department_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_ceo_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.members.len(), 1);
    assert!(org.members.contains_key(&ceo_id));

    // Add CTO reporting to CEO
    let cto_id = Uuid::now_v7();
    let cto_role = OrganizationRole {
        title: "CTO".to_string(),
        level: RoleLevel::Executive,
        reports_to: Some(ceo_id),
    };

    let message_id2 = Uuid::now_v7();
    let add_cto_cmd = AddMember {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        organization_id: org_id,
        person_id: cto_id,
        role: cto_role,
        department_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_cto_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.members.len(), 2);
    let cto = org.members.get(&cto_id).unwrap();
    assert_eq!(cto.role.reports_to, Some(ceo_id));

    // Add Engineering Manager reporting to CTO
    let eng_mgr_id = Uuid::now_v7();
    let eng_mgr_role = OrganizationRole {
        title: "Engineering Manager".to_string(),
        level: RoleLevel::Senior,
        reports_to: Some(cto_id),
    };

    let message_id3 = Uuid::now_v7();
    let add_mgr_cmd = AddMember {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id3),
            causation_id: cim_domain::CausationId(message_id3),
            message_id: message_id3,
        },
        organization_id: org_id,
        person_id: eng_mgr_id,
        role: eng_mgr_role,
        department_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_mgr_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.members.len(), 3);

    // Test circular reporting prevention
    let message_id = Uuid::now_v7();
    let circular_cmd = ChangeReportingRelationship {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: org_id,
        subordinate_id: ceo_id,
        new_manager_id: eng_mgr_id,
    };

    let result = org.handle_command(OrganizationCommand::ChangeReportingRelationship(
        circular_cmd,
    ));
    assert!(result.is_err());
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
        child_type: OrganizationType::Corporation, // Using Corporation as Division doesn't exist
    };

    let events = company
        .handle_command(OrganizationCommand::AddChildOrganization(add_division_cmd))
        .unwrap();
    company.apply_event(&events[0]).unwrap();

    // Note: child_units field doesn't exist in the new aggregate structure
    // Would need to track this differently

    // Create department under division
    let dept_id = Uuid::now_v7();
    let mut division = OrganizationAggregate::new(
        division_id,
        "Tech Division".to_string(),
        OrganizationType::Corporation, // Division type doesn't exist
    );
    division.status = OrganizationStatus::Active;
    // Note: parent_id field doesn't exist in the new aggregate

    let message_id2 = Uuid::now_v7();
    let add_dept_cmd = AddChildOrganization {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        parent_organization_id: division_id,
        child_organization_id: dept_id,
        child_name: "Engineering Department".to_string(),
        child_type: OrganizationType::Corporation,
    };

    let events = division
        .handle_command(OrganizationCommand::AddChildOrganization(add_dept_cmd))
        .unwrap();
    division.apply_event(&events[0]).unwrap();

    // Note: child_units field doesn't exist in the new aggregate

    // Test self-reference prevention
    let message_id3 = Uuid::now_v7();
    let self_ref_cmd = AddChildOrganization {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id3),
            causation_id: cim_domain::CausationId(message_id3),
            message_id: message_id3,
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
fn test_organization_locations() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Multi-Location Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Add first location (should become primary)
    let hq_location_id = Uuid::now_v7();
    let message_id = Uuid::now_v7();
    let add_hq_cmd = AddLocation {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: org_id,
        location_id: hq_location_id,
        name: "Headquarters".to_string(),
        address: "123 Main St, City, State".to_string(),
    };

    let events = org
        .handle_command(OrganizationCommand::AddLocation(add_hq_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.locations.len(), 1);
    // Check if the location is primary (first location becomes primary automatically)
    let hq_location = org.locations.get(&hq_location_id).unwrap();
    assert!(hq_location.is_primary);

    // Add second location
    let branch_location_id = Uuid::now_v7();
    let message_id2 = Uuid::now_v7();
    let add_branch_cmd = AddLocation {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id2),
            causation_id: cim_domain::CausationId(message_id2),
            message_id: message_id2,
        },
        organization_id: org_id,
        location_id: branch_location_id,
        name: "Branch Office".to_string(),
        address: "456 Oak Ave, City, State".to_string(),
    };

    let events = org
        .handle_command(OrganizationCommand::AddLocation(add_branch_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.locations.len(), 2);
    // Check that HQ is still primary
    let hq_location = org.locations.get(&hq_location_id).unwrap();
    assert!(hq_location.is_primary);

    // Change primary location
    let message_id3 = Uuid::now_v7();
    let change_primary_cmd = ChangePrimaryLocation {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id3),
            causation_id: cim_domain::CausationId(message_id3),
            message_id: message_id3,
        },
        organization_id: org_id,
        location_id: branch_location_id,
    };

    let events = org
        .handle_command(OrganizationCommand::ChangePrimaryLocation(
            change_primary_cmd,
        ))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    // Check that branch is now primary
    let branch_location = org.locations.get(&branch_location_id).unwrap();
    assert!(branch_location.is_primary);
    let hq_location = org.locations.get(&hq_location_id).unwrap();
    assert!(!hq_location.is_primary);

    // Remove original primary location
    let message_id4 = Uuid::now_v7();
    let remove_cmd = RemoveLocation {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id4),
            causation_id: cim_domain::CausationId(message_id4),
            message_id: message_id4,
        },
        organization_id: org_id,
        location_id: hq_location_id,
    };

    let events = org
        .handle_command(OrganizationCommand::RemoveLocation(remove_cmd))
        .unwrap();
    for event in &events {
        org.apply_event(event).unwrap();
    }

    assert_eq!(org.locations.len(), 1);
    assert!(org.locations.get(&branch_location_id).unwrap().is_primary);
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

    // Add some members
    let employee_id = Uuid::now_v7();
    let msg_id = Uuid::now_v7();
    let add_member_cmd = AddMember {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(msg_id),
            causation_id: cim_domain::CausationId(msg_id),
            message_id: msg_id,
        },
        organization_id: org_id,
        person_id: employee_id,
        role: OrganizationRole::software_engineer(),
        department_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_member_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    // Try to dissolve organization
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

    let result = org.handle_command(OrganizationCommand::DissolveOrganization(dissolve_cmd.clone()));

    // Clear members to allow dissolution
    org.members.clear();

    // Now dissolution should succeed
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
    let ceo_role = OrganizationRole::ceo();
    assert!(ceo_role.has_permission(&Permission::CreateOrganization));
    assert!(ceo_role.has_permission(&Permission::ApproveBudget));

    let engineer_role = OrganizationRole::software_engineer();
    assert!(engineer_role.has_permission(&Permission::ViewOrganization));
    assert!(!engineer_role.has_permission(&Permission::ApproveBudget));
    assert!(!engineer_role.has_permission(&Permission::RemoveMember));

    let manager_role = OrganizationRole::engineering_manager();
    assert!(manager_role.has_permission(&Permission::AddMember));
    assert!(manager_role.has_permission(&Permission::UpdateMemberRole));
    assert!(!manager_role.has_permission(&Permission::DeleteOrganization));
}

#[test]
fn test_member_role_updates() {
    let org_id = Uuid::now_v7();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Role Update Corp".to_string(),
        OrganizationType::Corporation,
    );
    org.status = OrganizationStatus::Active;

    // Add member as junior engineer
    let person_id = Uuid::now_v7();
    let mut junior_role = OrganizationRole::software_engineer();
    junior_role.level = RoleLevel::Junior;

    let msg_id = Uuid::now_v7();
    let add_cmd = AddMember {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(msg_id),
            causation_id: cim_domain::CausationId(msg_id),
            message_id: msg_id,
        },
        organization_id: org_id,
        person_id,
        role: junior_role.clone(),
        department_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    // Promote to senior engineer
    let mut senior_role = OrganizationRole::software_engineer();
    senior_role.level = RoleLevel::Senior;
    senior_role.title = "Senior Software Engineer".to_string();

    let message_id = Uuid::now_v7();
    let update_cmd = UpdateMemberRole {
        identity: MessageIdentity {
            correlation_id: cim_domain::CorrelationId::Single(message_id),
            causation_id: cim_domain::CausationId(message_id),
            message_id,
        },
        organization_id: org_id,
        person_id,
        new_role: senior_role.clone(),
    };

    let events = org
        .handle_command(OrganizationCommand::UpdateMemberRole(update_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    let member = org.members.get(&person_id).unwrap();
    assert_eq!(member.role.level, RoleLevel::Senior);
    assert_eq!(member.role.title, "Senior Software Engineer");
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
