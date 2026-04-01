# SIGINT-Pi Development Makefile
# Multi-architecture build and development environment
#
# Quick reference:
#   make setup      - Initial Docker buildx setup
#   make dev-pi     - Start Pi development container
#   make dev-deck   - Start Steam Deck development container
#   make build-all  - Build all release images
#   make test       - Run tests on both architectures

.PHONY: help setup dev-pi dev-deck build-all build-pi build-deck test clean logs shell-pi shell-deck release

# Default target
help:
	@echo "SIGINT-Pi Development Commands"
	@echo "=============================="
	@echo ""
	@echo "Setup:"
	@echo "  make setup          - Initialize Docker buildx for multi-arch"
	@echo ""
	@echo "Development:"
	@echo "  make dev-pi         - Start Pi dev environment (ARM64 emulated)"
	@echo "  make dev-deck       - Start Steam Deck dev environment (x86_64)"
	@echo "  make dev-both       - Start both dev environments"
	@echo "  make dev-full       - Start all services (dev + metrics + mqtt)"
	@echo ""
	@echo "Building:"
	@echo "  make build-pi       - Build Pi release image"
	@echo "  make build-deck     - Build Steam Deck release image"
	@echo "  make build-all      - Build all release images"
	@echo ""
	@echo "Testing:"
	@echo "  make test           - Run tests on both architectures"
	@echo "  make test-pi        - Run tests on ARM64"
	@echo "  make test-deck      - Run tests on x86_64"
	@echo "  make lint           - Run clippy and format checks"
	@echo ""
	@echo "Utilities:"
	@echo "  make shell-pi       - Open shell in Pi container"
	@echo "  make shell-deck     - Open shell in Deck container"
	@echo "  make logs-pi        - Follow Pi container logs"
	@echo "  make logs-deck      - Follow Deck container logs"
	@echo "  make clean          - Stop all containers and clean volumes"
	@echo ""
	@echo "Release:"
	@echo "  make release        - Build and export release binaries"
	@echo "  make package-pi     - Create Pi deployment package"
	@echo "  make package-deck   - Create Steam Deck deployment package"

# ============================================
# Setup
# ============================================

setup:
	@echo "Setting up Docker buildx for multi-architecture builds..."
	docker buildx create --use --name sigint-multiarch || true
	docker buildx inspect --bootstrap
	@echo "Installing QEMU for ARM64 emulation..."
	docker run --privileged --rm tonistiigi/binfmt --install all
	@echo ""
	@echo "Setup complete! You can now use:"
	@echo "  make dev-pi     - for Pi development"
	@echo "  make dev-deck   - for Steam Deck development"

# ============================================
# Development
# ============================================

dev-pi:
	@echo "Starting Pi Zero 2 W development environment (ARM64)..."
	docker compose -f docker-compose.dev.yml up pi-dev

dev-deck:
	@echo "Starting Steam Deck development environment (x86_64)..."
	docker compose -f docker-compose.dev.yml up deck-dev

dev-both:
	@echo "Starting both development environments..."
	docker compose -f docker-compose.dev.yml up pi-dev deck-dev

dev-full:
	@echo "Starting full development stack..."
	docker compose -f docker-compose.dev.yml --profile full up

dev-bg:
	@echo "Starting development containers in background..."
	docker compose -f docker-compose.dev.yml up -d pi-dev deck-dev

# ============================================
# Building
# ============================================

build-pi:
	@echo "Building Pi release image (ARM64)..."
	docker buildx build \
		--platform linux/arm64 \
		--target runtime \
		-f docker/Dockerfile.pi \
		-t sigint-pi:pi-latest \
		--load \
		.

build-deck:
	@echo "Building Steam Deck release image (x86_64)..."
	docker buildx build \
		--platform linux/amd64 \
		--target runtime \
		-f docker/Dockerfile.steamdeck \
		-t sigint-pi:deck-latest \
		--load \
		.

build-all: build-pi build-deck
	@echo "All release images built successfully!"
	docker images | grep sigint-pi

# Multi-platform image (push to registry)
build-multiarch:
	@echo "Building multi-architecture image..."
	docker buildx build \
		--platform linux/amd64,linux/arm64 \
		-f Dockerfile \
		-t sigint-pi:multiarch \
		--push \
		.

# ============================================
# Testing
# ============================================

test:
	@echo "Running tests on both architectures..."
	docker compose -f docker-compose.dev.yml --profile test up --abort-on-container-exit

test-deck:
	@echo "Running tests on x86_64..."
	docker compose -f docker-compose.dev.yml run --rm test-amd64

test-pi:
	@echo "Running tests on ARM64 (emulated)..."
	docker compose -f docker-compose.dev.yml run --rm test-arm64

lint:
	@echo "Running clippy and format checks..."
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings

# Local tests (native Mac)
test-local:
	cargo test --all-features

# ============================================
# Utilities
# ============================================

shell-pi:
	docker compose -f docker-compose.dev.yml exec pi-dev bash

shell-deck:
	docker compose -f docker-compose.dev.yml exec deck-dev bash

logs-pi:
	docker compose -f docker-compose.dev.yml logs -f pi-dev

logs-deck:
	docker compose -f docker-compose.dev.yml logs -f deck-dev

logs:
	docker compose -f docker-compose.dev.yml logs -f

status:
	@echo "Container Status:"
	docker compose -f docker-compose.dev.yml ps
	@echo ""
	@echo "Images:"
	docker images | grep sigint-pi

stop:
	docker compose -f docker-compose.dev.yml down

clean:
	@echo "Stopping all containers and removing volumes..."
	docker compose -f docker-compose.dev.yml down -v
	docker compose -f docker-compose.dev.yml --profile full down -v
	@echo "Removing build cache..."
	docker builder prune -f

clean-all: clean
	@echo "Removing all sigint-pi images..."
	docker rmi $$(docker images | grep sigint-pi | awk '{print $$3}') 2>/dev/null || true

# ============================================
# Release / Packaging
# ============================================

release: build-all
	@echo "Extracting release binaries..."
	mkdir -p dist/pi dist/deck
	docker create --name sigint-pi-extract sigint-pi:pi-latest
	docker cp sigint-pi-extract:/app/sigint-pi dist/pi/
	docker cp sigint-pi-extract:/app/static dist/pi/
	docker rm sigint-pi-extract
	docker create --name sigint-deck-extract sigint-pi:deck-latest
	docker cp sigint-deck-extract:/app/sigint-pi dist/deck/
	docker cp sigint-deck-extract:/app/static dist/deck/
	docker rm sigint-deck-extract
	@echo "Binaries extracted to dist/"
	ls -la dist/pi dist/deck

package-pi: build-pi
	@echo "Creating Pi deployment package..."
	mkdir -p release
	docker create --name pi-pkg sigint-pi:pi-latest
	docker cp pi-pkg:/app release/sigint-pi-arm64
	docker rm pi-pkg
	cp -r static release/sigint-pi-arm64/
	cp config.toml.example release/sigint-pi-arm64/config.toml
	cp -r data release/sigint-pi-arm64/
	cp scripts/pi-setup.sh release/sigint-pi-arm64/ 2>/dev/null || true
	cd release && tar -czvf sigint-pi-arm64.tar.gz sigint-pi-arm64
	@echo "Package created: release/sigint-pi-arm64.tar.gz"

package-deck: build-deck
	@echo "Creating Steam Deck deployment package..."
	mkdir -p release
	docker create --name deck-pkg sigint-pi:deck-latest
	docker cp deck-pkg:/app release/sigint-pi-amd64
	docker rm deck-pkg
	cp -r static release/sigint-pi-amd64/
	cp config.toml.example release/sigint-pi-amd64/config.toml
	cp -r data release/sigint-pi-amd64/
	cp -r steamdeck/* release/sigint-pi-amd64/ 2>/dev/null || true
	cd release && tar -czvf sigint-pi-amd64.tar.gz sigint-pi-amd64
	@echo "Package created: release/sigint-pi-amd64.tar.gz"

# ============================================
# Quick Development Workflows
# ============================================

# Start Pi dev and open browser
dev-pi-open: dev-bg
	@sleep 5
	@echo "Opening http://localhost:8080"
	open http://localhost:8080 || xdg-open http://localhost:8080 || true

# Watch and rebuild on changes (requires cargo-watch locally)
watch:
	cargo watch -x 'build --release'

# Database inspection
db-inspect:
	@echo "Inspecting Pi database..."
	docker compose -f docker-compose.dev.yml exec pi-dev sqlite3 /data/sigint.db ".tables"

db-dump:
	docker compose -f docker-compose.dev.yml exec pi-dev sqlite3 /data/sigint.db ".dump" > backup.sql
	@echo "Database dumped to backup.sql"

# ============================================
# CI/CD Helpers
# ============================================

ci-test:
	docker buildx build --platform linux/amd64 --target test -f docker/Dockerfile.steamdeck -t sigint-test .
	docker run --rm sigint-test

ci-build:
	docker buildx build --platform linux/amd64,linux/arm64 -f Dockerfile --push -t ghcr.io/sigint-pi/sigint-pi:latest .
