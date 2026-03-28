#!/bin/bash
# Run SIGINT-Pi on macOS using Docker
# Uses simulation mode for WiFi (macOS can't do monitor mode)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

echo "=================================="
echo "SIGINT-Pi for macOS"
echo "=================================="
echo ""

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "Error: Docker is not installed"
    echo "Install Docker Desktop from: https://docker.com/products/docker-desktop"
    exit 1
fi

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo "Error: Docker is not running"
    echo "Please start Docker Desktop"
    exit 1
fi

# Create config if not exists
if [ ! -f "config.toml" ]; then
    echo "Creating default config.toml..."
    cp config.toml.example config.toml
    echo ""
    echo "IMPORTANT: Edit config.toml to add your Telegram credentials:"
    echo "  nano config.toml"
    echo ""
fi

# Build and run
echo "Building SIGINT-Pi container..."
docker-compose build sigint-pi

echo ""
echo "Starting SIGINT-Pi in simulation mode..."
echo "Dashboard: http://localhost:8080"
echo ""
echo "Press Ctrl+C to stop"
echo ""

docker-compose up sigint-pi

