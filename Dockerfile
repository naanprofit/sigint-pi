# SIGINT-Pi Multi-Architecture Build
# Supports: x86_64 (Steam Deck, generic Linux) and arm64 (Raspberry Pi)
#
# Build for current platform:
#   docker build -t sigint-pi .
#
# Build for specific platform:
#   docker build --platform linux/amd64 -t sigint-pi:amd64 .
#   docker build --platform linux/arm64 -t sigint-pi:arm64 .

# ============================================
# Builder stage - compiles the Rust binary
# ============================================
FROM debian:bookworm-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    libpcap-dev \
    libbluetooth-dev \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .

# Build the project in release mode
RUN cargo build --release

# ============================================
# Runtime stage - minimal production image
# ============================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 \
    libdbus-1-3 \
    libpcap0.8 \
    libbluetooth3 \
    gpsd \
    gpsd-clients \
    iproute2 \
    wireless-tools \
    iw \
    curl \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Create directories
RUN mkdir -p /etc/sigint-pi /var/lib/sigint-pi/pcap /var/log/sigint-pi /data

WORKDIR /app

# Copy binary and static files from builder
COPY --from=builder /app/target/release/sigint-pi /app/sigint-pi
COPY --from=builder /app/static /app/static
COPY --from=builder /app/config.toml.example /etc/sigint-pi/config.toml

# Volumes for persistent data
VOLUME ["/data", "/etc/sigint-pi"]

# Expose web interface
EXPOSE 8080

# Environment defaults
ENV RUST_LOG=info
ENV SIGINT_DB_PATH=/data/sigint.db
ENV SIGINT_ACCEPT_DISCLAIMER=1

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s \
    CMD curl -f http://localhost:8080/api/status || exit 1

ENTRYPOINT ["/app/sigint-pi"]

# ============================================
# Development stage - for local development
# ============================================
FROM debian:bookworm-slim AS dev

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libdbus-1-dev \
    libpcap-dev \
    libbluetooth-dev \
    curl \
    git \
    sqlite3 \
    tcpdump \
    iw \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /etc/sigint-pi /var/lib/sigint-pi/pcap /data

WORKDIR /app

# Default: run in simulation mode for development
ENV SIGINT_SIMULATION=1
CMD ["cargo", "run", "--release"]
