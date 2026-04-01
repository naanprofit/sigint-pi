#!/bin/bash
# Cross-compile SIGINT-Pi for Raspberry Pi Zero 2 W (armv7)
# Run this on your development machine

set -e

TARGET="armv7-unknown-linux-gnueabihf"
BINARY_NAME="sigint-pi"

echo "==================================="
echo "Cross-compiling for Raspberry Pi"
echo "Target: $TARGET"
echo "==================================="

# Check if cross-compilation tools are installed
if ! command -v cross &> /dev/null; then
    echo "Installing cross..."
    cargo install cross
fi

# Add target
rustup target add $TARGET

# Build using cross (handles Docker-based cross compilation)
echo ""
echo "Building..."
cross build --release --target $TARGET

# Output info
BINARY="target/$TARGET/release/$BINARY_NAME"
if [ -f "$BINARY" ]; then
    echo ""
    echo "Build successful!"
    echo "Binary: $BINARY"
    echo "Size: $(du -h "$BINARY" | cut -f1)"
    echo ""
    echo "To deploy to your Pi:"
    echo "  scp $BINARY user@<device-ip>:/opt/sigint-pi/"
else
    echo "Build failed!"
    exit 1
fi
