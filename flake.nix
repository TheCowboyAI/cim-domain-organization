{
  description = "CIM Organization Domain - Scalable NATS event-sourced service for NixOS and nix-darwin";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    darwin = {
      url = "github:LnL7/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, nixos-generators, darwin, flake-utils, rust-overlay }:
    let
      flakeContext = {
        inherit (self) inputs;
        inherit self;
      };

      # System-specific outputs
      systemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          rustVersion = pkgs.rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };

          buildInputs = with pkgs; [
            openssl
            pkg-config
            protobuf
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          nativeBuildInputs = with pkgs; [
            rustVersion
            cargo-edit
            cargo-watch
          ];

          # Build organization-service binary
          organization-service = pkgs.rustPlatform.buildRustPackage {
            pname = "cim-domain-organization";
            version = "0.8.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [ pkg-config ];
            buildInputs = with pkgs; [ openssl ];

            cargoBuildFlags = [ "--bin" "organization-service" ];

            meta = with pkgs.lib; {
              description = "CIM Organization Domain Service - Event-sourced organization management via NATS";
              homepage = "https://github.com/thecowboyai/cim-domain-organization";
              license = licenses.mit;
            };
          };

        in
        {
          packages = {
            default = organization-service;
            organization-service = organization-service;

            # Library package (for development)
            lib = pkgs.rustPlatform.buildRustPackage {
              pname = "cim-domain-organization";
              version = "0.8.0";
              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              inherit buildInputs nativeBuildInputs;

              checkType = "debug";
              doCheck = false;
            };
          };

          apps = {
            default = {
              type = "app";
              program = "${organization-service}/bin/organization-service";
            };
            organization-service = {
              type = "app";
              program = "${organization-service}/bin/organization-service";
            };
          };

          devShells.default = pkgs.mkShell {
            inherit buildInputs nativeBuildInputs;

            shellHook = ''
              echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
              echo "CIM Organization Domain - Development Environment"
              echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
              echo ""
              echo "Rust: $(rustc --version)"
              echo ""
              echo "Available commands:"
              echo "  cargo build --bin organization-service  - Build service"
              echo "  cargo run --bin organization-service    - Run service"
              echo "  cargo test                               - Run tests"
              echo ""
              echo "  nix build .#organization-lxc            - Build Proxmox LXC"
              echo "  nix build .#organization-service        - Build binary"
              echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            '';
          };
        });

    in systemOutputs // {
      # NixOS modules
      nixosModules = {
        default = import ./deployment/nix/container.nix flakeContext;
        organization-service = import ./deployment/nix/container.nix flakeContext;
        container = import ./deployment/nix/container.nix flakeContext;
      };

      # NixOS configurations
      nixosConfigurations = {
        # Example: NixOS container
        organization-container = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            (import ./deployment/nix/container.nix flakeContext)
            {
              services.cim-domain-organization = {
                enable = true;
                natsUrl = "nats://10.0.0.41:4222";
                streamName = "ORGANIZATION_EVENTS";
              };
            }
          ];
        };

        # Example: Proxmox LXC
        organization-lxc = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            (import ./deployment/nix/container.nix flakeContext)
            {
              services.cim-domain-organization = {
                enable = true;
                natsUrl = "nats://10.0.0.41:4222";
                streamName = "ORGANIZATION_EVENTS";
                containerIp = "10.0.64.141";
                gateway = "10.0.64.1";
                prefixLength = 19;
                nameservers = [ "10.0.0.254" "1.1.1.1" ];
                sshKeys = [
                  "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDecTwCL7tc0mzBabAsFp1k9C3G30Nr+LIOE4MW4KWNO steele@thecowboy.ai"
                ];
              };
            }
          ];
        };
      };

      # Proxmox LXC and other formats
      packages = {
        x86_64-linux = {
          # Proxmox LXC container
          organization-lxc = nixos-generators.nixosGenerate {
            system = "x86_64-linux";
            format = "proxmox-lxc";
            modules = [
              (import ./deployment/nix/container.nix flakeContext)
              {
                services.cim-domain-organization = {
                  enable = true;
                  natsUrl = "nats://10.0.0.41:4222";
                  streamName = "ORGANIZATION_EVENTS";
                  containerIp = "10.0.64.141";
                  gateway = "10.0.64.1";
                  prefixLength = 19;
                  nameservers = [ "10.0.0.254" "1.1.1.1" ];
                  sshKeys = [
                    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDecTwCL7tc0mzBabAsFp1k9C3G30Nr+LIOE4MW4KWNO steele@thecowboy.ai"
                  ];
                };
              }
            ];
          };
        };

        # macOS support
        aarch64-darwin = {
          organization-service-darwin =
            let
              pkgs = import nixpkgs {
                system = "aarch64-darwin";
                overlays = [ (import rust-overlay) ];
              };
            in pkgs.rustPlatform.buildRustPackage {
              pname = "cim-domain-organization";
              version = "0.8.0";
              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [
                openssl
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];

              cargoBuildFlags = [ "--bin" "organization-service" ];
            };
        };

        x86_64-darwin = {
          organization-service-darwin =
            let
              pkgs = import nixpkgs {
                system = "x86_64-darwin";
                overlays = [ (import rust-overlay) ];
              };
            in pkgs.rustPlatform.buildRustPackage {
              pname = "cim-domain-organization";
              version = "0.8.0";
              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              nativeBuildInputs = with pkgs; [ pkg-config ];
              buildInputs = with pkgs; [
                openssl
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];

              cargoBuildFlags = [ "--bin" "organization-service" ];
            };
        };
      };

      # nix-darwin module
      darwinModules.default = { config, lib, pkgs, ... }:
        with lib;
        let
          cfg = config.services.cim-domain-organization;
        in {
          options.services.cim-domain-organization = {
            enable = mkEnableOption "CIM Organization Domain Service";

            natsUrl = mkOption {
              type = types.str;
              default = "nats://localhost:4222";
              description = "NATS server URL";
            };

            streamName = mkOption {
              type = types.str;
              default = "ORGANIZATION_EVENTS";
              description = "JetStream stream name";
            };

            logLevel = mkOption {
              type = types.str;
              default = "info";
              description = "Logging level";
            };
          };

          config = mkIf cfg.enable {
            launchd.daemons.cim-domain-organization = {
              serviceConfig = {
                ProgramArguments = [
                  "${self.packages.${pkgs.system}.organization-service-darwin}/bin/organization-service"
                ];
                EnvironmentVariables = {
                  NATS_URL = cfg.natsUrl;
                  STREAM_NAME = cfg.streamName;
                  LOG_LEVEL = cfg.logLevel;
                };
                KeepAlive = true;
                RunAtLoad = true;
                StandardErrorPath = "/var/log/cim-organization.log";
                StandardOutPath = "/var/log/cim-organization.log";
              };
            };
          };
        };
    };
}
