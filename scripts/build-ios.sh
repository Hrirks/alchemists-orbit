#!/bin/bash
set -e

# Build script for iOS
# This script builds the Rust library for iOS simulator and device

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
RUST_DIR="$PROJECT_ROOT/rust"
IOS_DIR="$PROJECT_ROOT/mobile/ios"

echo "Building Rust library for iOS..."

cd "$RUST_DIR"

# Build for iOS simulator (arm64)
echo "Building for iOS simulator (arm64)..."
cargo build --release --package alchemists-orbit-api --target aarch64-apple-ios-sim

# Build for iOS simulator (x86_64) 
echo "Building for iOS simulator (x86_64)..."
cargo build --release --package alchemists-orbit-api --target x86_64-apple-ios

# Build for iOS device
echo "Building for iOS device (arm64)..."
cargo build --release --package alchemists-orbit-api --target aarch64-apple-ios

# Create universal library for simulator
echo "Creating universal simulator library..."
mkdir -p "$RUST_DIR/target/ios-sim/release"
lipo -create \
    "$RUST_DIR/target/aarch64-apple-ios-sim/release/libalchemists_orbit_api.a" \
    "$RUST_DIR/target/x86_64-apple-ios/release/libalchemists_orbit_api.a" \
    -output "$RUST_DIR/target/ios-sim/release/libalchemists_orbit_api.a"

# Copy to iOS project
echo "Copying libraries to iOS project..."
mkdir -p "$IOS_DIR/libs"
cp "$RUST_DIR/target/ios-sim/release/libalchemists_orbit_api.a" "$IOS_DIR/libs/libalchemists_orbit_api_sim.a"
cp "$RUST_DIR/target/aarch64-apple-ios/release/libalchemists_orbit_api.a" "$IOS_DIR/libs/libalchemists_orbit_api_device.a"

echo "iOS Rust library build complete!"
