# SIGINT-Deck Alert Configuration Guide

SIGINT-Deck supports multiple alert channels to notify you when interesting devices are detected.

## Supported Alert Methods

| Method | Encryption | Requires Internet | Setup Difficulty |
|--------|-----------|-------------------|------------------|
| **Signal** | End-to-end ✓ | Yes | Medium |
| **Telegram** | Server-side | Yes | Easy |
| **Twilio SMS** | None | Yes | Easy |
| **Email** | TLS | Yes | Easy |
| **MQTT** | Optional TLS | LAN or Internet | Medium |
| **Webhooks** | HTTPS | Depends | Easy |
| **Sound** | N/A | No | Easy |

---

## Signal Messenger (Recommended for Security)

Signal provides end-to-end encryption - even Signal's servers can't read your alerts.

### Prerequisites

1. A phone number that can receive SMS (for initial registration)
2. Java runtime (signal-cli requirement)

### Installation (Steam Deck)

```bash
# Install Java runtime
sudo pacman -S jdk-openjdk

# Download signal-cli
cd /tmp
wget https://github.com/AsamK/signal-cli/releases/download/v0.13.4/signal-cli-0.13.4-Linux.tar.gz
tar xf signal-cli-*.tar.gz
sudo mv signal-cli-*/bin/signal-cli /usr/local/bin/
sudo mv signal-cli-*/lib /usr/local/lib/signal-cli
sudo chmod +x /usr/local/bin/signal-cli

# Verify installation
signal-cli --version
```

### Register Your Number

```bash
# Request verification code via SMS
signal-cli -a +1YOURNUMBER register

# Enter the code you receive
signal-cli -a +1YOURNUMBER verify 123456

# (Optional) Set a profile name
signal-cli -a +1YOURNUMBER updateProfile --given-name "SIGINT-Deck"
```

### Configuration

Add to `~/sigint-deck/config.toml`:

```toml
[alerts.signal]
enabled = true
sender_number = "+1YOURNUMBER"        # Your registered Signal number
recipients = ["+1RECIPIENT1"]          # Who receives alerts
signal_cli_path = "/usr/local/bin/signal-cli"
config_dir = "~/.local/share/signal-cli"
use_jsonrpc = false
min_priority = "high"                  # low, medium, high, critical
rate_limit_per_hour = 30
```

### Test Signal

```bash
# Send test message
signal-cli -a +1YOURNUMBER send -m "Test from SIGINT-Deck" +1RECIPIENT

# Via API (after starting SIGINT-Deck)
curl -X POST http://localhost:8080/api/test/signal
```

### Daemon Mode (Faster)

For faster message sending, run signal-cli as a daemon:

```bash
# Start daemon
signal-cli -a +1YOURNUMBER daemon --socket /tmp/signal-cli.sock &

# Update config
use_jsonrpc = true
jsonrpc_socket = "/tmp/signal-cli.sock"
```

---

## Telegram

Easy to set up, messages are encrypted to Telegram's servers.

### Setup

1. **Create a Bot**
   - Open Telegram and message `@BotFather`
   - Send `/newbot`
   - Follow prompts to name your bot
   - Save the **bot token** (looks like `123456789:ABCdefGHIjklMNOpqrsTUVwxyz`)

2. **Get Your Chat ID**
   - Message `@userinfobot` 
   - It will reply with your chat ID (a number)

### Configuration

```toml
[alerts.telegram]
enabled = true
bot_token = "123456789:ABCdefGHIjklMNOpqrsTUVwxyz"
chat_id = "987654321"
```

### Test

```bash
curl -X POST http://localhost:8080/api/test/telegram
```

---

## Twilio SMS

Standard SMS alerts via Twilio's API.

### Setup

1. Create account at https://twilio.com
2. Get a phone number
3. Note your Account SID and Auth Token

### Configuration

```toml
[alerts.twilio]
enabled = true
account_sid = "ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
auth_token = "your_auth_token"
from_number = "+1TWILIONUMBER"
to_number = "+1YOURNUMBER"
```

---

## Email

SMTP-based email alerts.

### Gmail Setup

1. Enable 2FA on your Google account
2. Create an App Password: Google Account → Security → App Passwords
3. Use the 16-character app password (not your regular password)

### Configuration

```toml
[alerts.email]
enabled = true
smtp_host = "smtp.gmail.com"
smtp_port = 587
smtp_user = "your-email@gmail.com"
smtp_password = "xxxx xxxx xxxx xxxx"  # App password
from_address = "your-email@gmail.com"
to_addresses = ["alerts@example.com"]
```

### Other Providers

| Provider | Host | Port |
|----------|------|------|
| Gmail | smtp.gmail.com | 587 |
| Outlook | smtp.office365.com | 587 |
| Yahoo | smtp.mail.yahoo.com | 587 |
| ProtonMail | 127.0.0.1 (bridge) | 1025 |

---

## MQTT

Publish alerts to an MQTT broker for home automation integration.

### Configuration

```toml
[alerts.mqtt]
enabled = true
broker_host = "mqtt.local"  # Or "localhost" for local broker
broker_port = 1883
client_id = "sigint-deck"
topic_prefix = "sigint"
# username = "optional"
# password = "optional"
```

### Topics Published

- `sigint/alert` - All alerts (JSON)
- `sigint/device/new` - New device detected
- `sigint/device/tracker` - Tracker detected
- `sigint/attack` - Attack detected

### Home Assistant Integration

```yaml
# configuration.yaml
mqtt:
  sensor:
    - name: "SIGINT Alerts"
      state_topic: "sigint/alert"
      value_template: "{{ value_json.title }}"
```

---

## Webhooks

Send alerts to any HTTP endpoint - great for custom integrations.

### Configuration

```toml
[alerts.webhook]
enabled = true

[[alerts.webhook.endpoints]]
name = "my-server"
url = "https://example.com/webhook"
method = "POST"
auth = { type = "bearer", token = "secret123" }
payload_format = "json"
min_priority = "medium"
timeout_secs = 30
```

### Payload Formats

**json** (default):
```json
{
  "priority": "high",
  "title": "Tracker Detected",
  "message": "AirTag found nearby",
  "device_mac": "AA:BB:CC:DD:EE:FF",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

**slack**:
```json
{
  "text": "🚨 *Tracker Detected*\nAirTag found nearby"
}
```

**discord**:
```json
{
  "content": "🚨 **Tracker Detected**\nAirTag found nearby"
}
```

---

## Sound Alerts

Local audio alerts through speakers/headphones.

### Configuration

```toml
[alerts.sound]
enabled = true
ninja_mode = false        # Set true to silence ALL sounds
volume = 70               # 0-100
new_device_sound = true   # Beep on new device
tracker_sound = true      # Alarm on tracker detection
attack_sound = true       # Alert on WiFi attacks
geofence_sounds = true    # Sound on location change
```

### Ninja Mode

When you need to be completely silent:

```toml
[alerts.sound]
enabled = true
ninja_mode = true  # Disables ALL audio alerts
```

Or via API:
```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"ninja_mode": true}' \
  http://localhost:8080/api/settings
```

---

## Alert Priority Levels

| Priority | When Used | Default Channels |
|----------|-----------|------------------|
| **Critical** | Tracker detected, Active attack | All |
| **High** | New unknown device, Strong signal | Signal, Telegram, Sound |
| **Medium** | Known device anomaly | Telegram, MQTT |
| **Low** | Routine events | MQTT, Webhook |

### Filtering by Priority

Each alert channel can set a minimum priority:

```toml
[alerts.signal]
min_priority = "critical"  # Only send for trackers/attacks

[alerts.telegram]
min_priority = "high"      # High and above

[alerts.mqtt]
# No min_priority = receives everything
```

---

## Testing Alerts

### Via API

```bash
# Test all configured channels
curl -X POST http://localhost:8080/api/test/alerts

# Test specific channel
curl -X POST http://localhost:8080/api/test/telegram
curl -X POST http://localhost:8080/api/test/signal
curl -X POST http://localhost:8080/api/test/email
```

### Via TUI

In TUI mode (`--tui`), press `t` to send a test alert.

---

## Troubleshooting

### Signal not sending

```bash
# Check signal-cli works
signal-cli -a +1YOURNUMBER send -m "Test" +1RECIPIENT

# Check registration
signal-cli -a +1YOURNUMBER listIdentities

# Re-register if needed
signal-cli -a +1YOURNUMBER register
signal-cli -a +1YOURNUMBER verify CODE
```

### Telegram "Unauthorized"

- Verify bot token is correct
- Make sure you've messaged the bot first (bots can't initiate conversations)
- Check chat_id is your user ID, not username

### Email "Authentication failed"

- Gmail: Must use App Password, not regular password
- Enable "Less secure apps" is deprecated - use App Passwords
- Check SMTP host and port

### MQTT "Connection refused"

- Verify broker is running: `mosquitto -v`
- Check firewall allows port 1883
- Test with: `mosquitto_pub -t test -m "hello"`
