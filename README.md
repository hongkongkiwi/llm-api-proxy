# LLM API Proxy

A transparent HTTP proxy for LLM APIs (Anthropic, OpenAI) with support for multiple named endpoints and proxy configurations.

## Features

- **Transparent Proxying**: Forwards requests to Anthropic and OpenAI APIs without modification
- **Multiple Named Endpoints**: Configure different proxy instances (dev, prod, staging, etc.)
- **TOML Configuration**: Easy-to-read configuration file format
- **Per-Endpoint Proxy Support**: Each endpoint can use different proxy servers
- **Flexible Target URLs**: Configure different target base URLs per endpoint
- **Environment Variable Support**: Override config file path with `CONFIG_PATH`

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (included with Rust)

### Build from Source

```bash
git clone git@github.com:hongkongkiwi/llm-api-proxy.git
cd llm-api-proxy
cargo build --release
# or using just
just build-release
```

### Run

```bash
cargo run
# or using just
just run

# with custom config
CONFIG_PATH=/path/to/config.toml cargo run

# with custom port using just
just run-port 8080
```

## Configuration

The proxy uses a TOML configuration file to define endpoints and their settings.

### Basic Configuration

Copy the example configuration file:

```bash
cp config.example.toml config.toml
```

Edit `config.toml` to configure your endpoints:

```toml
[server]
port = 8811
target_base = "https://api.anthropic.com"

# Anthropic API endpoints
[endpoints.anthropic_dev]
proxy_url = "http://localhost:8080"
target_base = "https://api.anthropic.com"

[endpoints.anthropic_prod]
proxy_url = "http://proxy.company.com:3128"
target_base = "https://api.anthropic.com"

# OpenAI API endpoints
[endpoints.openai_dev]
proxy_url = "http://localhost:8081"
target_base = "https://api.openai.com/v1"

[endpoints.openai_prod]
proxy_url = "http://proxy.company.com:3129"
target_base = "https://api.openai.com/v1"
```

### Configuration Options

#### Server Section

- `port`: Port to listen on (default: 8811)
- `target_base`: Default target base URL for all endpoints (can be overridden per endpoint)

#### Endpoint Sections

Each endpoint is defined as `[endpoints.{name}]`:

- `proxy_url`: Proxy server URL for this endpoint (optional)
- `target_base`: Target API base URL for this endpoint (optional, falls back to server.target_base)

## Usage

### Starting the Proxy

```bash
# Using default config.toml
cargo run

# Using custom config file
CONFIG_PATH=/path/to/custom-config.toml cargo run

# Using just command
just run
```

### Making Requests

Once the proxy is running, make requests to:

```
http://localhost:8811/{endpoint_name}/v1/{api_path}
```

#### Examples

**Anthropic API:**
```bash
curl -X POST http://localhost:8811/anthropic_dev/v1/messages \
  -H "Content-Type: application/json" \
  -H "x-api-key: your-anthropic-key" \
  -d '{
    "model": "claude-3-sonnet-20240229",
    "max_tokens": 100,
    "messages": [{"role": "user", "content": "Hello, world!"}]
  }'
```

**OpenAI API:**
```bash
curl -X POST http://localhost:8811/openai_prod/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-openai-key" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello, world!"}]
  }'
```

### Endpoint Routing

- `/anthropic_dev/v1/*` → Anthropic API through dev proxy
- `/anthropic_prod/v1/*` → Anthropic API through prod proxy
- `/openai_dev/v1/*` → OpenAI API through dev proxy
- `/openai_prod/v1/*` → OpenAI API through prod proxy

## Development

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Using just
just build
just build-release
```

### Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# Run E2E tests
cargo test --test e2e_tests

# Using just
just test
just test-unit
just test-integration
just test-e2e
just test-verbose
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Lint code
cargo clippy

# Using just
just fmt
just fmt-check
just lint
just lint-all
```

### Development Commands

```bash
# Run with hot reload (requires cargo-watch)
cargo watch -x run

# Run with custom port
cargo run -- --port 8080

# Using just
just run-port 8080
just run-dev
just run-with-proxy http://localhost:8080
```

### Project Management

```bash
# Clean build artifacts
cargo clean

# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Generate documentation
cargo doc --no-deps

# Open documentation in browser
cargo doc --no-deps --open

# Using just
just clean
just outdated
just update
just docs
just docs-open
```

### Docker Commands

```bash
# Build and run Docker container
docker build -t anthropic-http-proxy .
docker run -p 8811:8811 anthropic-http-proxy

# Using just
just docker-build
just docker-run
```

### Getting Help

```bash
# Show all available just commands
just --list

# Using just
just help
```

## System Startup

### Linux (systemd)

For automatic startup on Linux systems using systemd:

1. **Install the service:**
   ```bash
   sudo ./examples/install-linux.sh
   ```

2. **Manual installation:**
   ```bash
   # Copy the service file
   sudo cp examples/systemd.service /etc/systemd/system/anthropic-http-proxy.service
   
   # Create user and directories
   sudo useradd -r -s /bin/false anthropic-proxy
   sudo mkdir -p /opt/anthropic-http-proxy /etc/anthropic-http-proxy /var/log/anthropic-http-proxy
   sudo chown -R anthropic-proxy:anthropic-proxy /opt/anthropic-http-proxy /etc/anthropic-http-proxy /var/log/anthropic-http-proxy
   
   # Copy binary and config
   sudo cp target/release/anthropic-http-proxy /opt/anthropic-http-proxy/
   sudo cp config.toml /etc/anthropic-http-proxy/config.toml
   
   # Enable and start service
   sudo systemctl daemon-reload
   sudo systemctl enable anthropic-http-proxy
   sudo systemctl start anthropic-http-proxy
   ```

3. **Service management:**
   ```bash
   sudo systemctl status anthropic-http-proxy
   sudo systemctl stop anthropic-http-proxy
   sudo systemctl restart anthropic-http-proxy
   sudo journalctl -u anthropic-http-proxy -f
   ```

### macOS (launchd)

For automatic startup on macOS using launchd:

1. **Install the service:**
   ```bash
   sudo ./examples/install-macos.sh
   ```

2. **Manual installation:**
   ```bash
   # Copy the plist file
   sudo cp examples/launchd.plist /Library/LaunchDaemons/com.anthropic.http-proxy.plist
   
   # Create user and directories
   sudo sysadminctl -addUser anthropic-proxy
   sudo mkdir -p /opt/anthropic-http-proxy /etc/anthropic-http-proxy /var/log/anthropic-http-proxy
   sudo chown -R anthropic-proxy:anthropic-proxy /opt/anthropic-http-proxy /etc/anthropic-http-proxy /var/log/anthropic-http-proxy
   
   # Copy binary and config
   sudo cp target/release/anthropic-http-proxy /opt/anthropic-http-proxy/
   sudo cp config.toml /etc/anthropic-http-proxy/config.toml
   
   # Load and start service
   sudo launchctl load /Library/LaunchDaemons/com.anthropic.http-proxy.plist
   sudo launchctl start com.anthropic.http-proxy
   ```

3. **Service management:**
   ```bash
   sudo launchctl list | grep anthropic
   sudo launchctl stop com.anthropic.http-proxy
   sudo launchctl start com.anthropic.http-proxy
   tail -f /var/log/anthropic-http-proxy/output.log
   ```

### Windows (NSSM)

For automatic startup on Windows using NSSM (Non-Sucking Service Manager):

1. **Prerequisites:**
   - Download NSSM from https://nssm.cc
   - Add NSSM to your PATH or run from the same directory

2. **Install the service:**
   ```cmd
   # Run as Administrator
   examples\install-windows.bat
   ```

3. **Manual installation:**
   ```cmd
   # Install service using NSSM
   nssm install anthropic-http-proxy C:\opt\anthropic-http-proxy\anthropic-http-proxy.exe
   nssm set anthropic-http-proxy AppDirectory C:\opt\anthropic-http-proxy
   nssm set anthropic-http-proxy AppEnvironmentExtra "CONFIG_PATH=C:\opt\anthropic-http-proxy\config.toml" "PORT=8811"
   nssm set anthropic-http-proxy AppStdout C:\opt\anthropic-http-proxy\logs\output.log
   nssm set anthropic-http-proxy AppStderr C:\opt\anthropic-http-proxy\logs\error.log
   nssm set anthropic-http-proxy Start SERVICE_AUTO_START
   ```

4. **Service management:**
   ```cmd
   net start anthropic-http-proxy
   net stop anthropic-http-proxy
   sc query anthropic-http-proxy
   type C:\opt\anthropic-http-proxy\logs\output.log
   ```

### Docker

For containerized deployment:

```bash
# Build Docker image
just docker-build

# Run container with docker-compose
docker-compose up -d

# Run container manually
docker run -d \
  --name anthropic-http-proxy \
  -p 8811:8811 \
  -v $(pwd)/config.toml:/etc/anthropic-http-proxy/config.toml:ro \
  -v $(pwd)/logs:/var/log/anthropic-http-proxy \
  --restart unless-stopped \
  anthropic-http-proxy

# View logs
docker logs -f anthropic-http-proxy

# Stop container
docker stop anthropic-http-proxy
```

### Compiled Binary Usage

For running pre-compiled binaries:

1. **Download the latest release:**
   ```bash
   # Linux x86_64
   curl -L https://github.com/hongkongkiwi/llm-api-proxy/releases/latest/download/anthropic-http-proxy-linux-x86_64.tar.gz | tar -xz
   
   # macOS x86_64
   curl -L https://github.com/hongkongkiwi/llm-api-proxy/releases/latest/download/anthropic-http-proxy-macos-x86_64.tar.gz | tar -xz
   
   # Windows x86_64
   curl -L https://github.com/hongkongkiwi/llm-api-proxy/releases/latest/download/anthropic-http-proxy-windows-x86_64.zip -o anthropic-http-proxy.zip
   unzip anthropic-http-proxy.zip
   ```

2. **Make binary executable (Linux/macOS):**
   ```bash
   chmod +x anthropic-http-proxy
   ```

3. **Run the binary:**
   ```bash
   # With default config
   ./anthropic-http-proxy
   
   # With custom config
   CONFIG_PATH=/path/to/config.toml ./anthropic-http-proxy
   
   # With custom port
   PORT=8080 ./anthropic-http-proxy
   ```

4. **Install as system service (see System Startup section above)**

### Building Release Binaries

To build release binaries for all platforms:

```bash
# Build release packages for all platforms
just build-release-binaries

# Or run the script directly
./examples/build-release.sh

# This creates packages in the releases/ directory:
# - anthropic-http-proxy-VERSION-linux-x86_64.tar.gz
# - anthropic-http-proxy-VERSION-linux-aarch64.tar.gz
# - anthropic-http-proxy-VERSION-macos-x86_64.tar.gz
# - anthropic-http-proxy-VERSION-macos-aarch64.tar.gz
# - anthropic-http-proxy-VERSION-windows-x86_64.zip
```

### Version Management

To manage application versions:

```bash
# Bump patch version (1.0.0 -> 1.0.1)
just bump-patch

# Bump minor version (1.0.0 -> 1.1.0)
just bump-minor

# Bump major version (1.0.0 -> 2.0.0)
just bump-major

# Set specific version
just bump-version 1.2.3

# Create release tag and push
just release
```

The release process:
1. Bumps the version in Cargo.toml
2. Commits the version change
3. Creates a git tag
4. Pushes to remote repository
5. Triggers GitHub Actions to build and publish release artifacts

## Architecture

The proxy is designed to be completely transparent:

1. **Request Reception**: Receives HTTP requests on configured port
2. **Endpoint Routing**: Routes requests based on URL path (`/{endpoint}/v1/...`)
3. **Proxy Forwarding**: Forwards requests through configured proxy (if any)
4. **Target API**: Sends requests to the configured target API
5. **Response Return**: Returns responses back to the client

No API-specific logic or request/response modification is performed.

## Environment Variables

- `CONFIG_PATH`: Path to configuration file (default: `config.toml`)
- `PORT`: Server port (overrides config file, default: 8811)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass and code is formatted
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues and questions, please open an issue on the GitHub repository.