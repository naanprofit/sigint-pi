#!/bin/bash
# Run SIGINT-Pi with full metrics stack (InfluxDB + Grafana)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "=================================="
echo "SIGINT-Pi + Metrics Stack"
echo "=================================="
echo ""

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed"
    exit 1
fi

# Create config if not exists
if [ ! -f "config.toml" ]; then
    cp config.toml.example config.toml
fi

echo "Starting full stack..."
echo ""
echo "Services:"
echo "  - SIGINT-Pi Dashboard: http://localhost:8080"
echo "  - InfluxDB:            http://localhost:8086 (admin/sigintpi123)"
echo "  - Grafana:             http://localhost:3000 (admin/sigintpi)"
echo ""

docker-compose --profile metrics up --build

