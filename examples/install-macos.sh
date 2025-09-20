#!/bin/bash
# Installation script for macOS (launchd)
# Run as root: sudo ./install-macos.sh

set -e

# Configuration
SERVICE_USER="anthropic-proxy"
SERVICE_GROUP="anthropic-proxy"
INSTALL_DIR="/opt/anthropic-http-proxy"
CONFIG_DIR="/etc/anthropic-http-proxy"
LOG_DIR="/var/log/anthropic-http-proxy"
PLIST_FILE="/Library/LaunchDaemons/com.anthropic.http-proxy.plist"

echo "Installing anthropic-http-proxy as launchd service..."

# Create user and group
if ! id "$SERVICE_USER" &>/dev/null; then
    echo "Creating user and group: $SERVICE_USER"
    sysadminctl -addUser "$SERVICE_USER" 2>/dev/null || true
    dscl . -create /Groups/"$SERVICE_GROUP" 2>/dev/null || true
fi

# Create directories
echo "Creating directories..."
mkdir -p "$INSTALL_DIR" "$CONFIG_DIR" "$LOG_DIR"
chown -R "$SERVICE_USER:$SERVICE_GROUP" "$INSTALL_DIR" "$CONFIG_DIR" "$LOG_DIR"
chmod 755 "$INSTALL_DIR" "$CONFIG_DIR"
chmod 750 "$LOG_DIR"

# Copy binary (assuming it's built)
if [ -f "target/release/anthropic-http-proxy" ]; then
    echo "Copying binary..."
    cp target/release/anthropic-http-proxy "$INSTALL_DIR/"
    chown "$SERVICE_USER:$SERVICE_GROUP" "$INSTALL_DIR/anthropic-http-proxy"
    chmod 755 "$INSTALL_DIR/anthropic-http-proxy"
else
    echo "Warning: Binary not found at target/release/anthropic-http-proxy"
    echo "Please build the project first: cargo build --release"
fi

# Copy config file
if [ -f "config.toml" ]; then
    echo "Copying config file..."
    cp config.toml "$CONFIG_DIR/config.toml"
    chown "$SERVICE_USER:$SERVICE_GROUP" "$CONFIG_DIR/config.toml"
    chmod 640 "$CONFIG_DIR/config.toml"
else
    echo "Warning: config.toml not found in current directory"
    echo "Please create a config file at $CONFIG_DIR/config.toml"
fi

# Copy plist file
echo "Installing launchd service..."
cp examples/launchd.plist "$PLIST_FILE"
chown root:wheel "$PLIST_FILE"
chmod 644 "$PLIST_FILE"

# Load the service
launchctl load "$PLIST_FILE"

echo "Installation complete!"
echo ""
echo "To start the service:"
echo "  sudo launchctl start com.anthropic.http-proxy"
echo ""
echo "To check status:"
echo "  sudo launchctl list | grep anthropic"
echo ""
echo "To view logs:"
echo "  tail -f /var/log/anthropic-http-proxy/output.log"
echo "  tail -f /var/log/anthropic-http-proxy/error.log"