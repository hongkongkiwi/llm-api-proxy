#!/bin/bash
# Installation script for Linux (systemd)
# Run as root: sudo ./install-linux.sh

set -e

# Configuration
SERVICE_USER="anthropic-proxy"
SERVICE_GROUP="anthropic-proxy"
INSTALL_DIR="/opt/anthropic-http-proxy"
CONFIG_DIR="/etc/anthropic-http-proxy"
LOG_DIR="/var/log/anthropic-http-proxy"
SERVICE_FILE="/etc/systemd/system/anthropic-http-proxy.service"

echo "Installing anthropic-http-proxy as systemd service..."

# Create user and group
if ! id "$SERVICE_USER" &>/dev/null; then
    echo "Creating user and group: $SERVICE_USER"
    groupadd -r "$SERVICE_GROUP" 2>/dev/null || true
    useradd -r -g "$SERVICE_GROUP" -s /bin/false -d "$INSTALL_DIR" "$SERVICE_USER" 2>/dev/null || true
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

# Copy service file
echo "Installing systemd service..."
cp examples/systemd.service "$SERVICE_FILE"
systemctl daemon-reload
systemctl enable anthropic-http-proxy

echo "Installation complete!"
echo ""
echo "To start the service:"
echo "  sudo systemctl start anthropic-http-proxy"
echo ""
echo "To check status:"
echo "  sudo systemctl status anthropic-http-proxy"
echo ""
echo "To view logs:"
echo "  sudo journalctl -u anthropic-http-proxy -f"