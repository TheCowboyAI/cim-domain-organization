# CIM Domain - Organization

Organization bounded context for the Composable Information Machine.

## Overview

This module contains all organization-related domain logic including:
- Organization aggregate with hierarchical structure
- Organization membership and roles
- Organization metadata and status
- Command and query handlers for organization operations

## Structure

- `Organization` - The main aggregate root for organizations
- `OrganizationMember` - Value object representing a member of an organization
- `OrganizationType` - Enum for different types of organizations (Company, Department, Team, etc.)
- `OrganizationStatus` - Enum for organization lifecycle states
- `OrganizationRole` - Enum for member roles within an organization

## Commands

- `CreateOrganization` - Create a new organization
- `AddOrganizationMember` - Add a person as a member of an organization

## Events

- `OrganizationCreated` - Emitted when an organization is created
- `OrganizationMemberAdded` - Emitted when a member is added to an organization

## Usage

```rust
use cim_domain_organization::{
    Organization, CreateOrganization, OrganizationType,
    OrganizationCommandHandler
};
use uuid::Uuid;

// Create a new organization
let cmd = CreateOrganization {
    organization_id: Uuid::new_v4(),
    name: "Acme Corp".to_string(),
    org_type: OrganizationType::Company,
    parent_id: None,
    primary_location_id: None,
};

// Process through command handler
handler.handle(cmd).await?;
```

## Features

- `bevy`: Enable Bevy ECS integration for visualization

## License

See the main CIM repository for license information.
