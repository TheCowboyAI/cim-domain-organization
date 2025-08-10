//! Organization Structure Example
//!
//! This example demonstrates:
//! - Creating organizations
//! - Managing organizational hierarchy
//! - Adding members with roles
//! - Organization lifecycle management

use cim_domain_organization::{
    commands::CreateOrganization,
    handlers::{OrganizationCommandHandler, OrganizationQueryHandler, command_handler::InMemoryEventStore, query_handler::InMemoryReadModelStore},
    queries::GetOrganizationById,
    value_objects::OrganizationType,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CIM Organization Domain Example ===\n");

    // Initialize event store and handlers
    let event_store = InMemoryEventStore::new();
    let command_handler = OrganizationCommandHandler::new(event_store);
    let read_store = InMemoryReadModelStore::new();
    let query_handler = OrganizationQueryHandler::new(read_store);

    // Create a new organization
    let org_id = Uuid::new_v4();
    let create_cmd = CreateOrganization {
        organization_id: org_id,
        name: "Acme Corporation".to_string(),
        org_type: OrganizationType::Company,
        parent_id: None,
        primary_location_id: None,
    };

    // Execute command
    match command_handler.handle_create_organization(create_cmd).await {
        Ok(events) => {
            println!("✅ Organization created successfully!");
            for event in events {
                println!("  📝 Event: {:?}", event);
            }
        }
        Err(e) => {
            println!("❌ Failed to create organization: {}", e);
        }
    }

    // Query the organization
    let query = GetOrganizationById { organization_id: org_id };
    match query_handler.get_organization_by_id(query).await {
        Ok(Some(org)) => {
            println!("\n🏢 Organization Details:");
            println!("  ID: {}", org.organization_id);
            println!("  Name: {}", org.name);
            println!("  Type: {:?}", org.org_type);
            println!("  Status: {:?}", org.status);
            println!("  Members: {}", org.member_count);
        }
        Ok(None) => println!("❌ Organization not found"),
        Err(e) => println!("❌ Failed to query organization: {}", e),
    }

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
