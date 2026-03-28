# SIGINT-Pi Build and Test Environment
# Emulates Raspberry Pi OS (Debian-based ARM) for cross-compilation and testing

FROM --platform=linux/arm64 debian:bookworm-slim AS builder

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

# Build the project
RUN cargo build --release

# Runtime image
FROM --platform=linux/arm64 debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 \
    libdbus-1-3 \
    libpcap0.8 \
    libbluetooth3 \
    gpsd \
    gpsd-clients \
    iproute2 \
    wireless-tools \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Create directories
RUN mkdir -p /etc/sigint-pi /var/lib/sigint-pi/pcap /var/log/sigint-pi

WORKDIR /app

COPY --from=builder /app/target/release/sigint-pi /app/sigint-pi
COPY --from=builder /app/config.toml.example /etc/sigint-pi/config.toml

# Expose web interface
EXPOSE 8080

# Default command (with simulation mode)
ENV SIGINT_SIMULATION=1
CMD ["/app/sigint-pi"]

---
# Development/Test image with mock drivers
FROM --platform=linux/arm64 debian:bookworm-slim AS dev

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
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /etc/sigint-pi /var/lib/sigint-pi/pcap

WORKDIR /app

# For development, mount source as volume
CMD ["cargo", "run", "--features", "simulation"]
