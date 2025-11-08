# NixOS Container Configuration for CIM Organization Domain Service
#
# This module configures the organization-service to run in a container environment.
# Supports:
# - Proxmox LXC containers (via nixos-generators)
# - NixOS containers (via nixos-container)
# - Scalable replicas
#
# Usage:
#   1. Proxmox LXC:
#      nix build .#organization-lxc
#
#   2. NixOS container:
#      containers.organization-service = {
#        autoStart = true;
#        config = import ./deployment/nix/container.nix;
#      };

{ inputs, self, ... }@flakeContext:
{ config, lib, pkgs, modulesPath, ... }:

with lib;

let
  cfg = config.services.cim-domain-organization;

  # Build the organization-service binary
  organization-service = pkgs.rustPlatform.buildRustPackage rec {
    pname = "cim-domain-organization";
    version = "0.8.0";

    src = ../..;

    cargoLock = {
      lockFile = ../../Cargo.lock;
    };

    nativeBuildInputs = with pkgs; [ pkg-config ];
    buildInputs = with pkgs; [ openssl ];

    cargoBuildFlags = [ "--bin" "organization-service" ];

    meta = with lib; {
      description = "CIM Organization Domain Service";
      homepage = "https://github.com/thecowboyai/cim-domain-organization";
      license = licenses.mit;
    };
  };

in {
  imports = [
    # Only import Proxmox LXC module if it exists (container environments)
    (if builtins.pathExists (modulesPath + "/virtualisation/proxmox-lxc.nix")
     then (modulesPath + "/virtualisation/proxmox-lxc.nix")
     else {})
  ];

  options.services.cim-domain-organization = {
    enable = mkEnableOption "CIM Organization Domain Service";

    natsUrl = mkOption {
      type = types.str;
      default = "nats://10.0.0.41:4222";
      description = "NATS server URL";
    };

    streamName = mkOption {
      type = types.str;
      default = "ORGANIZATION_EVENTS";
      description = "JetStream stream name";
    };

    logLevel = mkOption {
      type = types.enum [ "trace" "debug" "info" "warn" "error" ];
      default = "info";
      description = "Logging level";
    };

    snapshotFrequency = mkOption {
      type = types.int;
      default = 100;
      description = "Snapshot frequency in events";
    };

    containerIp = mkOption {
      type = types.nullOr types.str;
      default = null;
      description = "Static IP address for container (optional)";
      example = "10.0.64.141";
    };

    gateway = mkOption {
      type = types.str;
      default = "10.0.64.1";
      description = "Default gateway";
    };

    prefixLength = mkOption {
      type = types.int;
      default = 19;
      description = "Network prefix length";
    };

    nameservers = mkOption {
      type = types.listOf types.str;
      default = [ "10.0.0.254" "1.1.1.1" ];
      description = "DNS nameservers";
    };

    sshKeys = mkOption {
      type = types.listOf types.str;
      default = [];
      description = "SSH authorized keys for root user";
      example = [ "ssh-ed25519 AAAAC3Nz... user@host" ];
    };
  };

  config = mkIf cfg.enable {
    # Container-specific settings
    boot.isContainer = mkDefault true;

    # Suppress systemd units that don't work in containers
    systemd.suppressedSystemUnits = mkDefault [
      "dev-mqueue.mount"
      "sys-kernel-debug.mount"
      "sys-fs-fuse-connections.mount"
    ];

    system.stateVersion = "24.05";

    networking = {
      hostName = "organization-service";
      domain = mkDefault "cim.local";

      enableIPv6 = mkDefault false;

      # Static IP configuration (if provided)
      defaultGateway = mkIf (cfg.containerIp != null) {
        address = cfg.gateway;
        interface = "eth0";
      };

      nameservers = cfg.nameservers;

      interfaces.eth0 = mkIf (cfg.containerIp != null) {
        useDHCP = false;
        ipv4.addresses = [{
          address = cfg.containerIp;
          prefixLength = cfg.prefixLength;
        }];
      };
    };

    # Firewall - allow outbound to NATS
    networking.firewall = {
      enable = mkDefault true;
      allowedTCPPorts = mkDefault [];  # No inbound ports needed
    };

    # SSH for management (if keys provided)
    services.openssh = mkIf (cfg.sshKeys != []) {
      enable = true;
      settings = {
        PermitRootLogin = "prohibit-password";
        PasswordAuthentication = false;
      };
    };

    users.users.root.openssh.authorizedKeys.keys = cfg.sshKeys;

    # Organization Domain Service
    systemd.services.cim-domain-organization = {
      description = "CIM Organization Domain Service";
      wantedBy = [ "multi-user.target" ];
      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];

      environment = {
        NATS_URL = cfg.natsUrl;
        STREAM_NAME = cfg.streamName;
        LOG_LEVEL = cfg.logLevel;
        SNAPSHOT_FREQ = toString cfg.snapshotFrequency;
      };

      serviceConfig = {
        Type = "simple";
        ExecStart = "${organization-service}/bin/organization-service";
        Restart = "always";
        RestartSec = "5s";

        # Security hardening
        DynamicUser = true;
        NoNewPrivileges = true;
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        ProtectKernelTunables = true;
        ProtectKernelModules = true;
        ProtectControlGroups = true;
        RestrictAddressFamilies = [ "AF_INET" "AF_INET6" "AF_UNIX" ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        LockPersonality = true;
        SystemCallFilter = "@system-service";
        SystemCallErrorNumber = "EPERM";
      };
    };

    # Minimal package set
    environment.systemPackages = with pkgs; [
      curl
      htop
      vim
      tmux
    ];

    # Logging
    services.journald.extraConfig = ''
      SystemMaxUse=100M
      MaxRetentionSec=7day
    '';
  };
}
