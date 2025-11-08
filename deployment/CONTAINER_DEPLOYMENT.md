# Container Deployment Guide - Organization Domain Service

This guide covers deploying the **organization-service** in containers for production scaling.

## Overview

The organization-service is designed to run in containers and scale horizontally:
- Multiple identical replicas
- All connect to same NATS cluster
- NATS handles load distribution
- JetStream provides shared event store
- No coordination between replicas needed

## Deployment Options

### 1. Proxmox LXC (Production) ✅ **Recommended**

Build and deploy LXC containers to Proxmox.

#### Build LXC Image

```bash
# From repository root
nix build .#organization-lxc

# Result: result/tarball/nixos-system-x86_64-linux.tar.xz
ls -lh result/tarball/
```

#### Deploy to Proxmox

```bash
# Copy to Proxmox host
scp result/tarball/*.tar.xz root@proxmox:/var/lib/vz/template/cache/organization-service.tar.xz

# SSH to Proxmox
ssh root@proxmox

# Create container (adjust ID and IP as needed)
pct create 141 \
  /var/lib/vz/template/cache/organization-service.tar.xz \
  --hostname organization-service-1 \
  --memory 512 \
  --cores 1 \
  --net0 name=eth0,bridge=vmbr0,ip=10.0.64.141/19,gw=10.0.64.1 \
  --nameserver 10.0.0.254 \
  --storage local-lxc \
  --unprivileged 1 \
  --features nesting=1 \
  --start 1

# Start container
pct start 141

# Check status
pct exec 141 -- systemctl status cim-domain-organization

# View logs
pct exec 141 -- journalctl -u cim-domain-organization -f
```

#### Scale Horizontally

Deploy multiple replicas:

```bash
# Replica 1
pct create 141 /var/lib/vz/template/cache/organization-service.tar.xz \
  --hostname organization-service-1 \
  --net0 name=eth0,bridge=vmbr0,ip=10.0.64.141/19,gw=10.0.64.1 \
  --start 1

# Replica 2
pct create 142 /var/lib/vz/template/cache/organization-service.tar.xz \
  --hostname organization-service-2 \
  --net0 name=eth0,bridge=vmbr0,ip=10.0.64.142/19,gw=10.0.64.1 \
  --start 1

# Replica 3
pct create 143 /var/lib/vz/template/cache/organization-service.tar.xz \
  --hostname organization-service-3 \
  --net0 name=eth0,bridge=vmbr0,ip=10.0.64.143/19,gw=10.0.64.1 \
  --start 1
```

All replicas connect to same NATS cluster (10.0.0.41:4222). NATS queue groups distribute commands automatically.

### 2. NixOS Containers (Pure NixOS)

For pure NixOS deployments without Proxmox.

#### Configuration

Add to your NixOS configuration:

```nix
# /etc/nixos/configuration.nix or flake-based config

{
  imports = [
    /path/to/cim-domain-organization/deployment/nix/container.nix
  ];

  containers.organization-service = {
    autoStart = true;
    privateNetwork = true;
    hostAddress = "10.0.64.1";
    localAddress = "10.0.64.141";

    config = { config, pkgs, ... }: {
      services.cim-domain-organization = {
        enable = true;
        natsUrl = "nats://10.0.0.41:4222";
        streamName = "ORGANIZATION_EVENTS";
        logLevel = "info";
        snapshotFrequency = 100;
      };
    };
  };
}
```

#### Manage Container

```bash
# Rebuild NixOS
sudo nixos-rebuild switch

# Check status
sudo systemctl status container@organization-service

# View logs
sudo journalctl -M organization-service -u cim-domain-organization -f

# Enter container
sudo nixos-container root-login organization-service
```

### 3. nix-darwin (macOS Development)

For local development on macOS.

#### Configuration

Add to your nix-darwin configuration:

```nix
# ~/.config/nix-darwin/configuration.nix or darwin-configuration.nix

{ config, pkgs, ... }:

{
  imports = [
    /path/to/cim-domain-organization/flake.nix.darwinModules.default
  ];

  services.cim-domain-organization = {
    enable = true;
    natsUrl = "nats://localhost:4222";
    streamName = "ORGANIZATION_EVENTS";
    logLevel = "debug";
  };
}
```

#### Manage Service

```bash
# Rebuild darwin configuration
darwin-rebuild switch

# Check status
launchctl list | grep organization

# View logs
log stream --predicate 'process == "organization-service"' --level debug

# Stop service
launchctl stop com.cim.organization-service

# Start service
launchctl start com.cim.organization-service
```

## Configuration Options

All deployment methods support these configuration options:

```nix
services.cim-domain-organization = {
  enable = true;                          # Enable the service
  natsUrl = "nats://10.0.0.41:4222";      # NATS server URL
  streamName = "ORGANIZATION_EVENTS";      # JetStream stream name
  logLevel = "info";                       # trace|debug|info|warn|error
  snapshotFrequency = 100;                 # Snapshot every N events

  # Container-specific (Proxmox/NixOS containers)
  containerIp = "10.0.64.141";            # Static IP for container
  gateway = "10.0.64.1";                   # Default gateway
  prefixLength = 19;                       # Network prefix length
  nameservers = ["10.0.0.254" "1.1.1.1"]; # DNS servers
  sshKeys = [                              # SSH authorized keys
    "ssh-ed25519 AAAAC3... user@host"
  ];
};
```

## Environment Variables

The service uses these environment variables (set automatically from config):

```bash
NATS_URL=nats://10.0.0.41:4222    # NATS server URL
STREAM_NAME=ORGANIZATION_EVENTS    # JetStream stream name
LOG_LEVEL=info                     # Logging level
SNAPSHOT_FREQ=100                  # Snapshot frequency
```

## Scaling Strategy

### Horizontal Scaling

1. **Deploy Multiple Replicas**
   - Each replica is identical
   - All connect to same NATS cluster
   - NATS queue groups distribute load

2. **No Coordination Required**
   - Stateless service design
   - All state in JetStream
   - Can add/remove replicas anytime

3. **Geographic Distribution**
   - Deploy replicas in different locations
   - Connect to regional NATS clusters
   - NATS super-clusters for global reach

### Load Distribution

NATS automatically distributes commands:
```
                    NATS Cluster
                   (10.0.0.41:4222)
                         |
         +---------------+---------------+
         |               |               |
    Container 1     Container 2     Container 3
    (10.0.64.141)   (10.0.64.142)   (10.0.64.143)
```

All subscribe to `organization.commands.>` in a queue group. NATS distributes messages round-robin.

## Monitoring & Health

### Check Service Health

```bash
# Proxmox LXC
pct exec 141 -- systemctl status cim-domain-organization

# NixOS Container
sudo systemctl status container@organization-service

# macOS (nix-darwin)
launchctl list | grep organization
```

### View Logs

```bash
# Proxmox LXC
pct exec 141 -- journalctl -u cim-domain-organization -f

# NixOS Container
sudo journalctl -M organization-service -u cim-domain-organization -f

# macOS (nix-darwin)
log stream --predicate 'process == "organization-service"' --level info
```

### NATS Metrics

Check NATS for service health:
```bash
# Check if service is connected
nats server report jetstream

# View stream stats
nats stream info ORGANIZATION_EVENTS

# Monitor message flow
nats sub "organization.events.>"
```

## Troubleshooting

### Service Won't Start

1. **Check NATS Connection**
   ```bash
   # Test NATS connectivity
   telnet 10.0.0.41 4222

   # Check NATS is running
   nats server ping
   ```

2. **Check JetStream**
   ```bash
   # Verify JetStream is enabled
   nats server info

   # Create stream manually if needed
   nats stream add ORGANIZATION_EVENTS --subjects "organization.events.>"
   ```

3. **View Service Logs**
   ```bash
   journalctl -u cim-domain-organization -n 100 --no-pager
   ```

### Container Network Issues

1. **Verify Network Configuration**
   ```bash
   # Check IP address
   ip addr show eth0

   # Test gateway
   ping -c 3 10.0.64.1

   # Test NATS server
   ping -c 3 10.0.0.41
   ```

2. **Check Firewall**
   ```bash
   # Proxmox host
   iptables -L -n -v
   ```

### High Memory Usage

1. **Adjust Snapshot Frequency**
   - Lower `snapshotFrequency` to reduce memory
   - Trade-off: more snapshots = slower event replay

2. **Scale Horizontally**
   - Add more replicas
   - Distribute load across containers

## Migration Path

Current → Future:

1. **Now: Proxmox LXC**
   - Deploy containers on Proxmox
   - Easy management, web UI
   - LXC overhead minimal

2. **Transition: Hybrid**
   - Some services in Proxmox LXC
   - Some in NixOS containers
   - Gradual migration

3. **Future: Pure NixOS**
   - All services as NixOS containers
   - Fully declarative
   - No Proxmox dependency

## Example: 3-Replica Production Deployment

```bash
# On Proxmox host

# Build image locally or copy pre-built
scp result/tarball/*.tar.xz root@proxmox:/var/lib/vz/template/cache/org.tar.xz

# Deploy 3 replicas
for i in 1 2 3; do
  ID=$((140 + i))
  IP="10.0.64.$ID"

  pct create $ID /var/lib/vz/template/cache/org.tar.xz \
    --hostname "organization-service-$i" \
    --memory 512 \
    --cores 1 \
    --net0 "name=eth0,bridge=vmbr0,ip=$IP/19,gw=10.0.64.1" \
    --nameserver "10.0.0.254" \
    --storage local-lxc \
    --unprivileged 1 \
    --features nesting=1 \
    --start 1

  echo "Created container $ID at $IP"
done

# Verify all running
pct list | grep organization

# Check all connected to NATS
for i in 141 142 143; do
  echo "=== Container $i ==="
  pct exec $i -- systemctl status cim-domain-organization | head -5
done
```

## Security Considerations

### Network Security

- **Firewall**: No inbound ports needed (outbound to NATS only)
- **SSH**: Optional, key-based authentication only
- **NATS**: Use TLS in production (configure `natsUrl` with `tls://`)

### Service Hardening

Container uses systemd security features:
- `DynamicUser`: Ephemeral user per service
- `ProtectSystem=strict`: Read-only filesystem
- `NoNewPrivileges`: Prevents privilege escalation
- `RestrictAddressFamilies`: Only AF_INET/INET6/UNIX
- System call filtering

### Secrets Management

For production, use:
1. **NixOps/Colmena**: Deploy secrets securely
2. **SOPS/Age**: Encrypt secrets in Nix config
3. **Vault**: External secrets management

## Performance Tuning

### Container Resources

Adjust based on load:

```bash
# Proxmox: Edit container config
pct set 141 --memory 1024 --cores 2

# NixOS: Update configuration
containers.organization-service.config = {
  virtualisation.memorySize = 1024;
  virtualisation.cores = 2;
};
```

### JetStream Tuning

Optimize stream configuration:
```bash
nats stream edit ORGANIZATION_EVENTS \
  --max-age=365d \
  --max-bytes=10G \
  --replicas=3
```

## Backup & Disaster Recovery

### Event Store Backup

JetStream stores all events:
```bash
# Backup JetStream directory
tar -czf organization-events-backup.tar.gz /path/to/jetstream/ORGANIZATION_EVENTS

# Or use NATS backup tools
nats stream backup ORGANIZATION_EVENTS ./backup-dir
```

### Container Backup

```bash
# Proxmox: Backup container
vzdump 141 --mode stop --compress gzip

# NixOS: Declarative config is the backup
git commit -m "organization-service config"
```

## Next Steps

1. **Deploy First Replica**
   - Test with single container
   - Verify NATS connection
   - Send test commands

2. **Scale to 3 Replicas**
   - Add 2 more containers
   - Verify load distribution
   - Monitor performance

3. **Production Hardening**
   - Enable TLS for NATS
   - Configure monitoring
   - Set up backups
   - Document runbooks

4. **Consider Pure NixOS**
   - Plan migration from Proxmox
   - Test NixOS containers
   - Declarative everything!

---

**The organization-service is now ready for production container deployment!** 🚀
