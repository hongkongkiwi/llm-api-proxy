#!/bin/bash

# Build script for creating release binaries
# This script builds the application for multiple platforms

set -e

# Configuration
APP_NAME="anthropic-http-proxy"
VERSION=${1:-$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)}
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-unknown-linux-gnu"
    "aarch64-apple-darwin"
)

echo "Building ${APP_NAME} v${VERSION} for multiple platforms..."

# Create release directory
mkdir -p releases

# Build for each target
for target in "${TARGETS[@]}"; do
    echo "Building for ${target}..."
    
    # Add target if not already installed
    rustup target add "${target}" || true
    
    # Build release
    cargo build --release --target "${target}"
    
    # Create platform-specific release package
    case "${target}" in
        *windows*)
            # Windows
            binary="target/${target}/release/${APP_NAME}.exe"
            package_name="${APP_NAME}-${VERSION}-windows-x86_64.zip"
            cd "target/${target}/release"
            zip "../../../../releases/${package_name}" "${APP_NAME}.exe"
            cd -
            ;;
        *darwin*)
            # macOS
            binary="target/${target}/release/${APP_NAME}"
            package_name="${APP_NAME}-${VERSION}-$(echo ${target} | cut -d'-' -f1)-$(echo ${target} | cut -d'-' -f3).tar.gz"
            cd "target/${target}/release"
            tar -czf "../../../../releases/${package_name}" "${APP_NAME}"
            cd -
            ;;
        *linux*)
            # Linux
            binary="target/${target}/release/${APP_NAME}"
            package_name="${APP_NAME}-${VERSION}-$(echo ${target} | cut -d'-' -f1)-$(echo ${target} | cut -d'-' -f3).tar.gz"
            cd "target/${target}/release"
            tar -czf "../../../../releases/${package_name}" "${APP_NAME}"
            cd -
            ;;
    esac
    
    echo "Created ${package_name}"
done

echo "Build complete! Release packages are in the 'releases' directory:"
ls -la releases/