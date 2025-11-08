# CIM Domain Organization - CT/FRP Conversion Assessment

**Date**: 2025-11-07
**Branch**: `ct-frp-conversion`
**Current Version**: 0.7.8
**Target Version**: 0.8.0

## Current State Analysis

### What's Already Good ✅

1. **Event Sourcing Foundation**
   - Well-defined Commands in `src/commands.rs`
   - Comprehensive Events in `src/events.rs`
   - Command → Events → State flow established

2. **Domain Model**
   - Rich domain entities: Organization, Department, Team, Role
   - Good separation of concerns
   - EntityId usage for type safety

3. **NATS Integration Started**
   - Has `src/nats/` module
   - Has `src/adapters/nats_event_publisher.rs`
   - Has `src/ports/event_publisher.rs`

4. **Dependencies**
   - Already has `cim-domain`, `async-nats`, `tokio`
   - Using `uuid` with v7 support

### What Needs Conversion ❌

#### 1. **Mutable Aggregate Operations**
**Current**: `aggregate.rs` uses mutable methods
```rust
pub fn handle_command(&mut self, command: OrganizationCommand) -> Result<Vec<Event>>
pub fn apply_event(&mut self, event: &OrganizationEvent) -> Result<()>
```

**Needed**: Pure functions
```rust
pub fn handle_command(self, command: OrganizationCommand) -> (Self, Vec<Event>)
pub fn apply_event_pure(&self, event: &OrganizationEvent) -> Result<Self>
```

#### 2. **MealyStateMachine Pattern Missing**
**Needed**: Implement `MealyStateMachine` trait from cim-domain
```rust
impl MealyStateMachine for OrganizationAggregate {
    type State = OrganizationState;
    type Input = OrganizationCommand;
    type Output = Vec<OrganizationEvent>;

    fn output(&self, state: Self::State, input: Self::Input) -> Self::Output;
    fn transition(&self, state: Self::State, input: Self::Input) -> Self::State;
}
```

#### 3. **No Service Binary**
**Needed**: Create `src/bin/organization-service.rs`
- Connect to NATS/JetStream
- Subscribe to `organization.commands.>`
- Publish events to `organization.events.>`
- Graceful shutdown
- Health checks

#### 4. **Container Deployment Missing**
**Needed**:
- `deployment/nix/container.nix` - Container module
- `deployment/CONTAINER_DEPLOYMENT.md` - Deployment guide
- Updated `flake.nix` with:
  - `nixosConfigurations.organization-container`
  - `nixosConfigurations.organization-lxc`
  - `packages.x86_64-linux.organization-lxc`
  - `packages.aarch64-darwin.organization-service-darwin`
  - `packages.x86_64-darwin.organization-service-darwin`
  - `darwinModules.default`

#### 5. **Service Binary Configuration in Cargo.toml**
**Needed**: Add binary definition
```toml
[[bin]]
name = "organization-service"
path = "src/bin/organization-service.rs"
```

## Conversion Strategy

### Phase 1: Pure Functions (Week 1)
1. Convert `OrganizationAggregate::handle_command` to pure function
2. Convert `OrganizationAggregate::apply_event` to `apply_event_pure`
3. Remove all `&mut self` references in aggregate
4. Ensure all methods return new instances instead of mutating

### Phase 2: MealyStateMachine (Week 1)
1. Define `OrganizationState` enum
2. Implement `MealyStateMachine` trait
3. Update command handlers to use state machine pattern

### Phase 3: NATS Service (Week 2)
1. Create `src/bin/organization-service.rs`
2. Implement JetStream event store
3. Add command subscription handlers
4. Add event publishing
5. Implement graceful shutdown

### Phase 4: Container Deployment (Week 2)
1. Create `deployment/nix/container.nix`
2. Update `flake.nix` with all outputs
3. Add nixos-generators for Proxmox LXC
4. Add nix-darwin support
5. Create `deployment/CONTAINER_DEPLOYMENT.md`

### Phase 5: Testing & Documentation (Week 3)
1. Unit tests for pure functions
2. Integration tests with NATS
3. Container build tests
4. Update README.md
5. Update CHANGELOG.md
6. Version bump to 0.8.0

## Key Changes Required

### 1. Aggregate Purity
**Before**:
```rust
fn handle_add_member(&mut self, cmd: AddMember) -> Result<Vec<Event>> {
    if self.members.contains_key(&cmd.person_id) {
        return Err(Error::DuplicateEntity);
    }
    Ok(vec![OrganizationEvent::MemberAdded(event)])
}
```

**After**:
```rust
fn handle_add_member(self, cmd: AddMember) -> (Self, Result<Vec<Event>>) {
    if self.members.contains_key(&cmd.person_id) {
        return (self, Err(Error::DuplicateEntity));
    }
    (self, Ok(vec![OrganizationEvent::MemberAdded(event)]))
}
```

### 2. Event Application
**Before**:
```rust
pub fn apply_event(&mut self, event: &OrganizationEvent) -> Result<()> {
    match event {
        OrganizationEvent::MemberAdded(e) => {
            self.members.insert(e.person_id, member);
        }
        // ...
    }
    self.version += 1;
    Ok(())
}
```

**After**:
```rust
pub fn apply_event_pure(&self, event: &OrganizationEvent) -> Result<Self> {
    let mut new_aggregate = self.clone();
    match event {
        OrganizationEvent::MemberAdded(e) => {
            new_aggregate.members.insert(e.person_id, member);
        }
        // ...
    }
    new_aggregate.version += 1;
    Ok(new_aggregate)
}
```

## Validation Checklist

After conversion, verify:

### Domain Layer
- [ ] No `&mut self` methods in aggregate
- [ ] All state changes return new instances
- [ ] MealyStateMachine implemented
- [ ] Pure functions only (no I/O)
- [ ] All events immutable
- [ ] Using `Uuid::now_v7()` everywhere

### NATS Integration
- [ ] Service binary created
- [ ] Subscribes to `organization.commands.>`
- [ ] Publishes to `organization.events.>`
- [ ] JetStream event store
- [ ] Graceful shutdown implemented

### Container Deployment
- [ ] Can build: `nix build .#organization-service`
- [ ] Can build: `nix build .#organization-lxc`
- [ ] Container module works for Proxmox
- [ ] Container module works for NixOS
- [ ] nix-darwin module works for macOS

### Testing
- [ ] All tests pass
- [ ] Zero compiler warnings
- [ ] Service runs successfully
- [ ] Can handle NATS commands
- [ ] Events published to JetStream

## Timeline

- **Week 1 (Days 1-5)**: Phases 1-2 (Pure functions & State machine)
- **Week 2 (Days 6-10)**: Phases 3-4 (NATS service & Container deployment)
- **Week 3 (Days 11-15)**: Phase 5 (Testing, documentation, release)

## Success Criteria

Conversion is complete when:
- ✅ All domain logic is pure functions
- ✅ MealyStateMachine pattern implemented
- ✅ NATS service runs successfully
- ✅ Can build and deploy LXC container
- ✅ All tests pass
- ✅ Zero compiler warnings
- ✅ Documentation updated
- ✅ Version 0.8.0 released

## Notes

- This is a comprehensive domain with many entities and complex relationships
- Careful testing needed for circular reference detection
- Status transition validation must remain strict
- Consider breaking into smaller commits for easier review
