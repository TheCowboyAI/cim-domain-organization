# CIM Domain Organization Module Context

## Purpose
The `cim-domain-organization` module is a core component of the Composable Information Machine (CIM) ecosystem, responsible for managing organizational entities, structures, and their associated data using an Entity Component System (ECS) architecture.

## Architecture Overview
- **Pattern**: ECS (Entity Component System)
- **Core Design**: Minimal Organization aggregate with composable components
- **Event Sourcing**: CQRS pattern with event-driven state management
- **Cross-Domain**: Integrates with Person, Agent, Location, Identity, and Git domains
- **Dependencies**: Uses `cim-domain-person` for membership and organizational units

## Key Concepts
1. **Organization Aggregate**: Minimal core containing only ID, name, and lifecycle state
2. **Components**: All other organization data (structure, units, membership, policies) managed as separate components
3. **Organizational Units**: Collections of people with specific roles and responsibilities
4. **Membership Management**: Tracks person-organization relationships and roles
5. **Projections**: Multiple read models (Hierarchy, Search, Membership, Summary, Timeline)
6. **Events**: Domain events for state changes and cross-domain integration

## Module Structure
```
src/
├── aggregate/       # Core organization entity and lifecycle
├── commands/        # Command handlers and validation
├── components/      # Organization data components (units, membership, etc.)
├── cross_domain/    # Integration with other CIM domains (especially Person)
├── events/          # Domain events and handlers
├── infrastructure/  # Storage, messaging, persistence
├── projections/     # Read models and views
├── queries/         # Query handlers
├── services/        # Business logic services
└── units/           # Organizational unit management and person collections
```

## Integration Points
- **Person Domain** (DEPENDENCY): Employment, membership, organizational unit composition
- **Agent Domain** (DEPENDENCY): AI agents as organizational members or automation
- **Identity Domain**: Authentication and identity management
- **Location Domain**: Organization-location associations
- **Git Domain**: Development activity tracking

## Organizational Unit Concepts
- **Units as Person Collections**: Departments, teams, squads are collections of people
- **Role-Based Membership**: People have roles within units (manager, member, lead, etc.)
- **Hierarchical Structure**: Units can contain sub-units, forming organizational trees
- **Cross-Functional Teams**: People can belong to multiple units with different roles

## Development Focus
When working on this module:
1. Maintain clean separation between aggregate and components
2. Use event sourcing for all state changes
3. Ensure cross-domain events are properly handled (especially Person events)
4. Follow ECS patterns for new features
5. Keep projections updated with domain changes
6. Remember organizational units are person collections - always reference PersonId