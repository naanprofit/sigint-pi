# OpenClaw Mesh Networking Integration

SIGINT-Deck supports OpenClaw mesh networking for distributed sensor deployment and data aggregation.

---

## Overview

OpenClaw is a mesh networking protocol designed for SIGINT/security applications, enabling:

- **Distributed Sensing** - Multiple SIGINT nodes sharing data
- **Resilient Communication** - Mesh topology survives node failures
- **Data Aggregation** - Central dashboard collecting from all nodes
- **Alert Propagation** - Threats detected by one node alert all nodes
- **GPS Correlation** - Location-aware device tracking across nodes

---

## Architecture

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│  SIGINT-Pi  │◄────►│  SIGINT-Pi  │◄────►│ SIGINT-Deck │
│   Node A    │      │   Node B    │      │   (Mobile)  │
└──────┬──────┘      └──────┬──────┘      └──────┬──────┘
       │                    │                    │
       └────────────────────┼────────────────────┘
                            │
                     ┌──────▼──────┐
                     │   OpenClaw  │
                     │   Gateway   │
                     └──────┬──────┘
                            │
                     ┌──────▼──────┐
                     │  Dashboard  │
                     │  (Web UI)   │
                     └─────────────┘
```

---

## Features

### Device Correlation
- Track devices seen across multiple nodes
- Build movement patterns from multi-point observations
- Correlate signal strength for triangulation

### Alert Mesh
- When one node detects a threat, all nodes are notified
- Configurable alert propagation (immediate, batched, filtered)
- Priority-based alert routing

### Data Synchronization
- Shared device database across all nodes
- Conflict resolution for concurrent updates
- Offline operation with sync-on-reconnect

### Meshtastic Bridge
- Optional LoRa bridge via Meshtastic devices
- Long-range communication without WiFi/cellular
- See [Meshtastic Integration](#meshtastic-integration) below

---

## Configuration

### Enable OpenClaw

Edit `config.toml`:

```toml
[openclaw]
enabled = true
node_id = "pi-living-room"      # Unique node identifier
role = "sensor"                  # sensor | gateway | relay

# Mesh network settings
mesh_port = 9000
discovery_port = 9001
encryption_key = "your-32-char-encryption-key-here"

# Gateway address (if this is a sensor node)
gateway_address = "192.168.1.100:9000"

# Data sharing settings
share_wifi_devices = true
share_ble_devices = true
share_alerts = true
share_gps = true
```

### Node Roles

| Role | Description |
|------|-------------|
| `sensor` | Collects data, sends to gateway |
| `gateway` | Aggregates data from sensors, serves dashboard |
| `relay` | Forwards traffic, extends mesh range |

### Gateway Configuration

```toml
[openclaw]
enabled = true
node_id = "gateway-main"
role = "gateway"

# Gateway-specific
web_dashboard = true
dashboard_port = 8888
aggregate_data = true
store_history = true

# MQTT bridge (optional)
mqtt_bridge = true
mqtt_broker = "mqtt://localhost:1883"
mqtt_topic_prefix = "sigint/"
```

---

## API Endpoints

### Node Status
```bash
GET /api/openclaw/status
```

Response:
```json
{
  "enabled": true,
  "node_id": "pi-living-room",
  "role": "sensor",
  "connected_peers": 3,
  "gateway_connected": true,
  "last_sync": "2024-01-15T10:30:00Z"
}
```

### List Peers
```bash
GET /api/openclaw/peers
```

### Sync Now
```bash
POST /api/openclaw/sync
```

### Get Aggregated Data (Gateway only)
```bash
GET /api/openclaw/aggregate/devices
GET /api/openclaw/aggregate/alerts
```

---

## Meshtastic Integration

For areas without WiFi/cellular coverage, OpenClaw can bridge via Meshtastic LoRa radios.

### Hardware Requirements

- Meshtastic-compatible device (T-Beam, T-Echo, Heltec, etc.)
- Connected via USB or Bluetooth to SIGINT node

### Configuration

```toml
[meshtastic]
enabled = true
device = "/dev/ttyUSB0"         # USB serial
# device = "bluetooth://XX:XX:XX:XX:XX:XX"  # Bluetooth

# Channel settings (must match other Meshtastic devices)
channel_name = "SIGINT-MESH"
channel_psk = "base64-encoded-psk"

# Data to send over LoRa (bandwidth limited!)
send_alerts = true              # High-priority alerts
send_new_devices = true         # New device notifications
send_gps = false                # GPS updates (high bandwidth)
send_device_count = true        # Periodic device count summaries

# Compression
compress_messages = true
```

### Alert Format

Meshtastic messages use a compact format:
```
ALERT|timestamp|type|mac|rssi|location
NEW|timestamp|mac|vendor|type
COUNT|wifi_count|ble_count|alert_count
```

---

## Security Considerations

### Encryption

All OpenClaw traffic is encrypted using:
- AES-256-GCM for data in transit
- ChaCha20-Poly1305 for Meshtastic bridge
- Shared encryption key configured per mesh

### Authentication

- Nodes authenticate via pre-shared key
- Optional certificate-based authentication for gateways
- Node ID spoofing protection via challenge-response

### Data Privacy

- MAC addresses can be hashed before sharing
- Location data can be fuzzed/excluded
- Configurable data retention policies

---

## Deployment Scenarios

### Home Security Mesh

```
Living Room Pi ◄──WiFi──► Bedroom Pi ◄──WiFi──► Garage Pi
       │                        │                    │
       └────────────────────────┼────────────────────┘
                                │
                         Gateway (NAS)
                                │
                          Dashboard
```

### Mobile + Fixed Deployment

```
Fixed Pi (Home) ◄──Meshtastic──► Mobile Deck (Car)
       │
   Dashboard
```

### Multi-Site with VPN

```
Site A Gateway ◄──VPN──► Site B Gateway
       │                        │
  Local Sensors            Local Sensors
```

---

## Troubleshooting

### Nodes Not Discovering

```bash
# Check discovery port
netstat -an | grep 9001

# Check firewall
sudo iptables -L | grep 900

# Enable discovery broadcast
sysctl net.ipv4.icmp_echo_ignore_broadcasts=0
```

### Gateway Connection Failed

```bash
# Test connectivity
ping <gateway-ip>
nc -zv <gateway-ip> 9000

# Check encryption key matches
grep encryption_key /var/lib/sigint-pi/config.toml
```

### Meshtastic Not Working

```bash
# Check device
ls /dev/ttyUSB*
ls /dev/ttyACM*

# Check permissions
sudo usermod -a -G dialout $USER

# Test with meshtastic CLI
pip install meshtastic
meshtastic --info
```

---

## Compatibility Notes

### Supported Platforms

| Platform | OpenClaw | Meshtastic Bridge |
|----------|----------|-------------------|
| Raspberry Pi | Full | Full |
| Steam Deck | Full | USB only (no BT) |
| macOS | Partial | Full |
| Docker | Full | USB passthrough |

### Network Requirements

| Port | Protocol | Purpose |
|------|----------|---------|
| 9000/TCP | OpenClaw | Mesh data |
| 9001/UDP | OpenClaw | Discovery broadcast |
| 1883/TCP | MQTT | Optional bridge |

### Bandwidth Considerations

| Data Type | ~Size | Recommended |
|-----------|-------|-------------|
| Device update | 200B | WiFi/LAN |
| Alert | 100B | Any |
| GPS position | 50B | Any |
| Full sync | 10KB+ | WiFi/LAN only |

---

## Future Roadmap

- [ ] Automatic mesh topology optimization
- [ ] Multi-hop routing for extended range
- [ ] Satellite backhaul support (Starlink, Iridium)
- [ ] Integration with RTL-SDR triangulation
- [ ] Distributed spectrum analysis
- [ ] Kubernetes deployment support

---

## See Also

- [Meshtastic Project](https://meshtastic.org/)
- [MQTT Bridge Configuration](ALERTS.md#mqtt)
- [Network Security](LEGAL.md)
