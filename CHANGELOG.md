# Changelog

All notable changes to the CIM Organization Domain will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2025-11-07

### Added
- **Pure Functional Architecture**: Complete conversion to pure functions following Category Theory (CT) and Functional Reactive Programming (FRP) principles
  - `apply_event_pure(&self) → Result<Self>` method for pure event application
  - All domain logic now side-effect free
  - Backward-compatible `apply_event(&mut self)` wrapper

- **MealyStateMachine Implementation**: Formal state machine pattern for organizational lifecycle
  - `OrganizationState` enum (Creating, Pending, Active, Inactive, Suspended, Dissolved, Merged)
  - Pure `output(State, Command) → Events` function
  - Pure `transition(State, Command) → NewState` function
  - Clear separation of state transitions from business logic

- **NATS Service Binary**: Production-ready event-sourced service
  - `organization-service` binary for NATS-based deployment
  - `NatsEventStore` with JetStream integration
  - `OrganizationCommandHandler` for command processing
  - `OrganizationRepository` with event sourcing and snapshot support
  - Environment-based configuration (NATS_URL, STREAM_NAME, LOG_LEVEL, SNAPSHOT_FREQ)
  - Graceful shutdown with signal handling
  - Comprehensive tracing/logging support

- **Container Deployment Support**: Multi-platform deployment options
  - **Proxmox LXC**: Build with `nix build .#organization-lxc`
  - **NixOS Containers**: Native NixOS container support
  - **nix-darwin**: macOS launchd service configuration
  - `deployment/nix/container.nix` - Full NixOS container module
  - `deployment/CONTAINER_DEPLOYMENT.md` - Comprehensive deployment guide
  - Security hardening with systemd features (DynamicUser, ProtectSystem, etc.)
  - Configurable via service options (natsUrl, streamName, logLevel, snapshotFrequency)

- **Flake Outputs**: Complete Nix flake structure
  - `nixosModules.{default,organization-service,container}` - NixOS service modules
  - `nixosConfigurations.{organization-container,organization-lxc}` - Example configurations
  - `packages.x86_64-linux.organization-lxc` - Proxmox LXC tarball builder
  - `packages.{aarch64,x86_64}-darwin.organization-service-darwin` - macOS binaries
  - `darwinModules.default` - nix-darwin launchd service
  - Enhanced development shell with build instructions

### Changed
- **Event Application**: Migrated from mutable to pure functional approach
  - All 17+ event handlers now use pure functions
  - No more `&mut self` mutations in domain logic
  - Events return new aggregate instances instead of mutating existing ones

- **Dependencies**: Added infrastructure dependencies
  - `async-nats 0.38` - NATS client library
  - `futures 0.3` - Async stream utilities
  - `tracing-subscriber` - Structured logging

- **Flake Structure**: Complete rewrite from 65 to 312 lines
  - Added `nixos-generators` input for LXC image building
  - Added `darwin` input for nix-darwin support
  - Multi-platform package outputs
  - Enhanced development environment

### Fixed
- Compilation warnings reduced to 2 non-critical unused field warnings
- Git tracking for deployment files (needed for Nix flake evaluation)

### Architecture
This release represents a fundamental architectural shift to pure functional programming:

**Before (0.7.x)**:
- Mutable aggregates with `&mut self` methods
- Side effects mixed with business logic
- No formal state machine
- No NATS service deployment

**After (0.8.0)**:
- 100% pure functions in domain layer
- MealyStateMachine for state transitions
- Event sourcing with NATS JetStream
- Production-ready container deployment
- Horizontal scaling via NATS queue groups

### Migration Guide
For users upgrading from 0.7.x:

1. **Existing Code**: Continues to work via backward-compatible wrappers
2. **New Code**: Use `apply_event_pure()` for pure functional approach
3. **NATS Deployment**: Optional - library can still be used standalone
4. **Container Deployment**: Optional - service can run via `cargo run --bin organization-service`

### References
- Conversion Assessment: `CONVERSION_ASSESSMENT.md`
- Conversion Progress: `CONVERSION_PROGRESS.md`
- Deployment Guide: `deployment/CONTAINER_DEPLOYMENT.md`
- Template: Based on `cim-domain-person` v0.8.0

## [0.7.8] - Previous Release
Previous functionality maintained for backward compatibility.

---

**Note**: This release maintains 100% backward compatibility while introducing pure functional architecture. Existing code using mutable methods will continue to work.
