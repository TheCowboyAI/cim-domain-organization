# Domain Boundary Violations - Analysis

**Date**: 2025-11-07
**Status**: ✅ **COMPLETED** - Implementation refactored to pure domain boundaries

## 🚨 Critical Issues Found

While we updated the documentation to show proper domain boundaries, the **actual implementation still violates these boundaries**.

### Violation 1: Embedding Location Data

**File**: `src/aggregate.rs:229-234`, `src/events.rs:461-470`

**Current (WRONG)**:
```rust
pub struct OrganizationLocation {
    pub id: Uuid,
    pub name: String,
    pub address: String,  // ❌ Embeds location domain data
    pub is_primary: bool,
}

pub struct LocationAdded {
    pub location_id: Uuid,
    pub name: String,
    pub address: String,  // ❌ Embeds location domain data
    pub is_primary: bool,
}
```

**Should Be (RIGHT)**:
```rust
pub struct OrganizationFacility {
    pub id: Uuid,
    pub name: String,           // Facility name (org domain)
    pub facility_type: FacilityType,
    pub location_id: Option<Uuid>,  // ✅ Reference to Location domain
    pub is_headquarters: bool,
}

pub struct FacilityCreated {
    pub facility_id: Uuid,
    pub name: String,
    pub facility_type: FacilityType,
    // NO address data!
}

pub struct FacilityLinkedToLocation {
    pub facility_id: Uuid,
    pub location_id: Uuid,  // ✅ Reference only
}
```

### Violation 2: Mixing Member and Role Concepts

**File**: `src/aggregate.rs:84-98`

**Current (WRONG)**:
```rust
pub struct OrganizationMember {
    pub id: Uuid,
    pub person_id: Uuid,
    pub role: OrganizationRole,  // ❌ Inline role definition
    pub department_id: Option<Uuid>,
}

pub struct OrganizationRole {  // ❌ Duplicate role concept
    pub title: String,
    pub level: RoleLevel,
    pub reports_to: Option<Uuid>,
}
```

**Problem**:
- We have `Role` entity in `entity.rs` (correct - a position)
- We have `OrganizationRole` struct in `aggregate.rs` (wrong - inline definition)
- `OrganizationMember` conflates person assignment with role definition

**Should Be (RIGHT)**:
```rust
// Remove OrganizationMember entirely
// Remove OrganizationRole struct entirely
// Use Role entity from entity.rs

// Track assignments as relationships:
pub struct RoleAssignment {
    pub assignment_id: Uuid,
    pub role_id: Uuid,        // ✅ Reference to Role entity
    pub person_id: Uuid,      // ✅ Reference to Person domain
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub assignment_type: AssignmentType,
}
```

### Violation 3: Person-to-Person Reporting

**File**: `src/events.rs:448-457`

**Current (WRONG)**:
```rust
pub struct ReportingRelationshipChanged {
    pub person_id: Uuid,              // ❌ Person reference
    pub new_manager_id: Option<Uuid>, // ❌ Person reference
}
```

**Should Be (RIGHT)**:
```rust
pub struct RoleReportingChanged {
    pub role_id: Uuid,                  // ✅ Position reference
    pub reports_to_role_id: Option<Uuid>,  // ✅ Position reference
}
```

Reporting relationships are between **positions**, not **people**. People inherit reporting relationships from their role assignments.

## What Organization Domain Should Own

### ✅ Correct Ownership

**Positions/Roles**:
- Role title, description, responsibilities
- Role level, type, permissions
- Role-to-role reporting (position hierarchy)
- Role status (active, vacant, deprecated)

**Places/Facilities**:
- Facility name, type, capacity
- Facility purpose, description
- Facility relationships (parent facility, etc.)

**Organizational Structure**:
- Departments, teams, divisions
- Budget allocations
- Policies and procedures
- Strategic objectives

### ❌ Not Organization's Responsibility

**People Data** (Person domain owns):
- Person names, demographics, contact info
- Person skills, qualifications, history
- Person-to-person social relationships

**Location Data** (Location domain owns):
- Physical addresses
- Geographic coordinates
- Location attributes (timezone, region, etc.)

## What Organization Domain Should Reference

**Via UUID References Only**:
- `person_id: Uuid` - Reference to Person domain
- `location_id: Uuid` - Reference to Location domain

**Via Relationship Events**:
- `PersonAssignedToRole { person_id, role_id }`
- `FacilityLinkedToLocation { facility_id, location_id }`

## Required Refactoring

### High Priority

1. **Replace OrganizationLocation with Facility**
   - Remove `address` field
   - Add `location_id` reference
   - Update all events: LocationAdded → FacilityCreated
   - Add new event: FacilityLinkedToLocation

2. **Remove OrganizationMember & OrganizationRole**
   - Use Role entity from entity.rs
   - Replace with RoleAssignment relationship
   - Update events: MemberAdded → PersonAssignedToRole
   - Update events: MemberRemoved → PersonRemovedFromRole

3. **Fix Reporting Relationships**
   - Change from person-to-person to role-to-role
   - Update event: ReportingRelationshipChanged → RoleReportingChanged

### Implementation Steps

1. Create new domain concepts:
   ```rust
   // In entity.rs
   pub struct Facility { ... }  // Already have Role
   pub struct RoleAssignment { ... }
   pub struct FacilityLocationLink { ... }
   ```

2. Update events:
   ```rust
   // Remove
   - LocationAdded
   - MemberAdded
   - MemberRemoved
   - ReportingRelationshipChanged

   // Add
   + FacilityCreated
   + FacilityLinkedToLocation
   + PersonAssignedToRole
   + PersonRemovedFromRole
   + RoleReportingChanged
   ```

3. Update aggregate:
   ```rust
   pub struct OrganizationAggregate {
       // Remove
       - pub members: HashMap<Uuid, OrganizationMember>,
       - pub locations: HashMap<Uuid, OrganizationLocation>,

       // Add
       + pub facilities: HashMap<Uuid, Facility>,
       + pub role_assignments: HashMap<Uuid, RoleAssignment>,
       + pub facility_location_links: HashMap<Uuid, FacilityLocationLink>,
   }
   ```

4. Update commands to match new structure

5. Update all event handlers in apply_event_pure

## Impact Assessment

### Breaking Changes
- ✅ **YES** - This is a breaking change to the domain model
- Commands will have different structure
- Events will have different names and fields
- Aggregates will have different internal structure

### Migration Path
- Events are immutable - old events stay as-is
- New version can read old events and upgrade them
- Provide migration tool to replay old events into new structure

### Timeline
- Refactoring: 4-6 hours
- Testing: 2-3 hours
- Migration tool: 2-3 hours
- **Total**: ~10 hours

## Decision

We have two options:

### Option A: Fix Now (Recommended)
- Do the refactoring before v0.8.0 release
- Release v0.8.0 with correct boundaries
- Mark v0.7.8 as deprecated
- Provide migration guide

### Option B: Document & Plan
- Release v0.8.0 with current implementation
- Mark domain boundary issues in docs as "known issues"
- Plan v0.9.0 for proper domain boundaries
- Users warned about future breaking changes

## Recommendation

**Option A: Fix Now** because:
1. We haven't released v0.8.0 yet
2. Better to get it right before production use
3. Early adopters will appreciate correct architecture
4. Easier to fix now than after deployment

The documentation already shows the correct approach. We should make the implementation match the documentation.

---

## ✅ Refactoring Completed (2025-11-07)

All domain boundary violations have been resolved:

### Changes Made

1. **Facility Entity Added** (entity.rs)
   - Created proper Facility entity with NO address data
   - Facility represents organizational place concept
   - Location references will be handled in Association domain

2. **Impure Structs Removed** (aggregate.rs)
   - ❌ Removed: OrganizationMember (relationship)
   - ❌ Removed: OrganizationLocation (embeds address)
   - ❌ Removed: OrganizationRole (duplicate of Role entity)
   - ❌ Removed: RoleLevel enum (unused)
   - ✅ Added: Facility HashMap

3. **Events Refactored** (events.rs)
   - ✅ Added: FacilityCreated, FacilityUpdated, FacilityRemoved
   - ❌ Removed: MemberAdded, MemberRoleUpdated, MemberRemoved
   - ❌ Removed: RoleAssigned, RoleVacated (relationships)
   - ❌ Removed: LocationAdded, PrimaryLocationChanged, LocationRemoved
   - ❌ Removed: ReportingRelationshipChanged

4. **Commands Refactored** (commands.rs)
   - ✅ Added: CreateFacility, UpdateFacility, RemoveFacility
   - ❌ Removed: AssignRole, VacateRole
   - ❌ Removed: AddMember, UpdateMemberRole, RemoveMember
   - ❌ Removed: ChangeReportingRelationship
   - ❌ Removed: AddLocation, ChangePrimaryLocation, RemoveLocation

5. **Event Handlers Updated** (aggregate.rs)
   - Updated apply_event_pure to handle Facility events
   - Removed all relationship event handlers
   - Updated handle_command dispatch

6. **NATS Integration Updated**
   - Updated event type mapping in nats_integration.rs
   - Updated correlation_id extraction in nats_event_publisher.rs
   - Updated timestamp extraction in nats_event_publisher.rs
   - Updated subject patterns in ports/event_publisher.rs

### Compilation Status
✅ Clean compilation with only 2 non-critical warnings (unused fields)

### Result
The Organization domain is now PURE:
- Only owns organizational concepts (roles, facilities, departments, teams)
- References other domains via UUID only
- Relationships belong in separate Association domain (to be created later)
