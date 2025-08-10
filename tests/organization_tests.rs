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
    // Create organization
    let org_id = Uuid::new_v4();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Acme Corporation".to_string(),
        OrganizationType::Company,
    );

    // Verify initial state
    assert_eq!(org.name, "Acme Corporation");
    assert_eq!(org.org_type, OrganizationType::Company);
    assert_eq!(org.status, OrganizationStatus::Pending);
    assert!(org.members.is_empty());
    assert!(org.locations.is_empty());

    // Create organization command
    let create_cmd = CreateOrganization {
        organization_id: org_id,
        name: "Acme Corporation".to_string(),
        org_type: OrganizationType::Company,
        parent_id: None,
        primary_location_id: None,
    };

    let events = org
        .handle_command(OrganizationCommand::Create(create_cmd))
        .unwrap();
    assert_eq!(events.len(), 1);

    // Apply event
    org.apply_event(&events[0]).unwrap();
    assert_eq!(org.status, OrganizationStatus::Active);
}

#[test]
fn test_organization_member_management() {
    let org_id = Uuid::new_v4();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Tech Startup".to_string(),
        OrganizationType::Company,
    );
    org.status = OrganizationStatus::Active;

    // Add CEO
    let ceo_id = Uuid::new_v4();
    let ceo_role = OrganizationRole::ceo();

    let add_ceo_cmd = AddMember {
        organization_id: org_id,
        person_id: ceo_id,
        role: ceo_role,
        reports_to: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_ceo_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.members.len(), 1);
    assert!(org.members.contains_key(&ceo_id));

    // Add CTO reporting to CEO
    let cto_id = Uuid::new_v4();
    let cto_role = OrganizationRole::cto();

    let add_cto_cmd = AddMember {
        organization_id: org_id,
        person_id: cto_id,
        role: cto_role,
        reports_to: Some(ceo_id),
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_cto_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.members.len(), 2);
    let cto = org.members.get(&cto_id).unwrap();
    assert_eq!(cto.reports_to, Some(ceo_id));

    // Add Engineering Manager reporting to CTO
    let eng_mgr_id = Uuid::new_v4();
    let eng_mgr_role = OrganizationRole::engineering_manager();

    let add_mgr_cmd = AddMember {
        organization_id: org_id,
        person_id: eng_mgr_id,
        role: eng_mgr_role,
        reports_to: Some(cto_id),
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_mgr_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.members.len(), 3);

    // Test circular reporting prevention
    let circular_cmd = ChangeReportingRelationship {
        organization_id: org_id,
        person_id: ceo_id,
        new_manager_id: Some(eng_mgr_id),
    };

    let result = org.handle_command(OrganizationCommand::ChangeReportingRelationship(
        circular_cmd,
    ));
    assert!(result.is_err());
}

#[test]
fn test_organization_hierarchy() {
    // Create parent company
    let company_id = Uuid::new_v4();
    let mut company = OrganizationAggregate::new(
        company_id,
        "Global Corp".to_string(),
        OrganizationType::Company,
    );
    company.status = OrganizationStatus::Active;

    // Create division
    let division_id = Uuid::new_v4();
    let add_division_cmd = AddChildOrganization {
        parent_id: company_id,
        child_id: division_id,
    };

    let events = company
        .handle_command(OrganizationCommand::AddChildOrganization(add_division_cmd))
        .unwrap();
    company.apply_event(&events[0]).unwrap();

    assert!(company.child_units.contains(&division_id));

    // Create department under division
    let dept_id = Uuid::new_v4();
    let mut division = OrganizationAggregate::new(
        division_id,
        "Tech Division".to_string(),
        OrganizationType::Division,
    );
    division.status = OrganizationStatus::Active;
    division.parent_id = Some(company_id);

    let add_dept_cmd = AddChildOrganization {
        parent_id: division_id,
        child_id: dept_id,
    };

    let events = division
        .handle_command(OrganizationCommand::AddChildOrganization(add_dept_cmd))
        .unwrap();
    division.apply_event(&events[0]).unwrap();

    assert!(division.child_units.contains(&dept_id));

    // Test self-reference prevention
    let self_ref_cmd = AddChildOrganization {
        parent_id: company_id,
        child_id: company_id,
    };

    let result = company.handle_command(OrganizationCommand::AddChildOrganization(self_ref_cmd));
    assert!(result.is_err());
}

#[test]
fn test_organization_locations() {
    let org_id = Uuid::new_v4();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Multi-Location Corp".to_string(),
        OrganizationType::Company,
    );
    org.status = OrganizationStatus::Active;

    // Add first location (should become primary)
    let hq_location_id = Uuid::new_v4();
    let add_hq_cmd = AddLocation {
        organization_id: org_id,
        location_id: hq_location_id,
        make_primary: false, // Should still become primary as first location
    };

    let events = org
        .handle_command(OrganizationCommand::AddLocation(add_hq_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.locations.len(), 1);
    assert_eq!(org.primary_location_id, Some(hq_location_id));

    // Add second location
    let branch_location_id = Uuid::new_v4();
    let add_branch_cmd = AddLocation {
        organization_id: org_id,
        location_id: branch_location_id,
        make_primary: false,
    };

    let events = org
        .handle_command(OrganizationCommand::AddLocation(add_branch_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.locations.len(), 2);
    assert_eq!(org.primary_location_id, Some(hq_location_id)); // Primary unchanged

    // Change primary location
    let change_primary_cmd = ChangePrimaryLocation {
        organization_id: org_id,
        new_location_id: branch_location_id,
    };

    let events = org
        .handle_command(OrganizationCommand::ChangePrimaryLocation(
            change_primary_cmd,
        ))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.primary_location_id, Some(branch_location_id));

    // Remove original primary location
    let remove_cmd = RemoveLocation {
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
    assert_eq!(org.primary_location_id, Some(branch_location_id));
}

#[test]
fn test_organization_status_transitions() {
    let org_id = Uuid::new_v4();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Status Test Corp".to_string(),
        OrganizationType::Company,
    );

    // Initial status is Pending
    assert_eq!(org.status, OrganizationStatus::Pending);

    // Activate organization
    org.status = OrganizationStatus::Active;

    // Test valid transition: Active -> Inactive
    let inactive_cmd = ChangeOrganizationStatus {
        organization_id: org_id,
        new_status: OrganizationStatus::Inactive,
        reason: Some("Temporary closure".to_string()),
    };

    let events = org
        .handle_command(OrganizationCommand::ChangeStatus(inactive_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();
    assert_eq!(org.status, OrganizationStatus::Inactive);

    // Test invalid transition: Inactive -> Merged (must go through Active)
    let invalid_cmd = ChangeOrganizationStatus {
        organization_id: org_id,
        new_status: OrganizationStatus::Merged,
        reason: None,
    };

    let result = org.handle_command(OrganizationCommand::ChangeStatus(invalid_cmd));
    assert!(result.is_err());

    // Test valid transition: Inactive -> Active
    let reactivate_cmd = ChangeOrganizationStatus {
        organization_id: org_id,
        new_status: OrganizationStatus::Active,
        reason: Some("Reopening".to_string()),
    };

    let events = org
        .handle_command(OrganizationCommand::ChangeStatus(reactivate_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();
    assert_eq!(org.status, OrganizationStatus::Active);
}

#[test]
fn test_organization_dissolution() {
    let org_id = Uuid::new_v4();
    let mut org = OrganizationAggregate::new(
        org_id,
        "To Be Dissolved Corp".to_string(),
        OrganizationType::Company,
    );
    org.status = OrganizationStatus::Active;

    // Add some members
    let employee_id = Uuid::new_v4();
    let add_member_cmd = AddMember {
        organization_id: org_id,
        person_id: employee_id,
        role: OrganizationRole::software_engineer(),
        reports_to: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_member_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    // Try to dissolve with child organizations (should fail)
    let child_id = Uuid::new_v4();
    org.child_units.insert(child_id);

    let dissolve_cmd = DissolveOrganization {
        organization_id: org_id,
        reason: "Bankruptcy".to_string(),
        member_disposition: MemberDisposition::Terminated,
    };

    let result = org.handle_command(OrganizationCommand::Dissolve(dissolve_cmd.clone()));
    assert!(result.is_err());

    // Remove child organizations
    org.child_units.clear();

    // Now dissolution should succeed
    let events = org
        .handle_command(OrganizationCommand::Dissolve(dissolve_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    assert_eq!(org.status, OrganizationStatus::Dissolved);
}

#[test]
fn test_organization_merger() {
    let source_id = Uuid::new_v4();
    let target_id = Uuid::new_v4();

    let mut source_org = OrganizationAggregate::new(
        source_id,
        "Small Startup".to_string(),
        OrganizationType::Company,
    );
    source_org.status = OrganizationStatus::Active;

    // Test merger
    let merge_cmd = MergeOrganizations {
        source_organization_id: source_id,
        target_organization_id: target_id,
        member_disposition: MemberDisposition::TransferredTo(target_id),
    };

    let events = source_org
        .handle_command(OrganizationCommand::Merge(merge_cmd))
        .unwrap();
    source_org.apply_event(&events[0]).unwrap();

    assert_eq!(source_org.status, OrganizationStatus::Merged);

    // Test self-merge prevention
    let self_merge_cmd = MergeOrganizations {
        source_organization_id: source_id,
        target_organization_id: source_id,
        member_disposition: MemberDisposition::Terminated,
    };

    let result = source_org.handle_command(OrganizationCommand::Merge(self_merge_cmd));
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
    let org_id = Uuid::new_v4();
    let mut org = OrganizationAggregate::new(
        org_id,
        "Role Update Corp".to_string(),
        OrganizationType::Company,
    );
    org.status = OrganizationStatus::Active;

    // Add member as junior engineer
    let person_id = Uuid::new_v4();
    let mut junior_role = OrganizationRole::software_engineer();
    junior_role.level = RoleLevel::Junior;

    let add_cmd = AddMember {
        organization_id: org_id,
        person_id,
        role: junior_role.clone(),
        reports_to: None,
    };

    let events = org
        .handle_command(OrganizationCommand::AddMember(add_cmd))
        .unwrap();
    org.apply_event(&events[0]).unwrap();

    // Promote to senior engineer
    let mut senior_role = OrganizationRole::software_engineer();
    senior_role.level = RoleLevel::Senior;
    senior_role.title = "Senior Software Engineer".to_string();

    let update_cmd = UpdateMemberRole {
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
    assert_eq!(max, Some(5000.0));
}
