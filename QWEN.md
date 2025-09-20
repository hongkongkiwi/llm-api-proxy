# QWEN.md - Context for anthropic-http-proxy

This document provides comprehensive context about the `anthropic-http-proxy` project for future interactions.

## Project Overview

The `anthropic-http-proxy` is a transparent HTTP proxy for LLM APIs (Anthropic, OpenAI) written in Rust. It enables serving multiple proxy endpoints through a single port with path-based routing, allowing requests to be forwarded to different target APIs through various proxy configurations.

### Key Features
- **Single Port Operation**: All endpoints served through one port (default: 8811)
- **Path-Based Routing**: Routes requests based on URL path prefix (`/{endpoint_name}/v1/*`)
- **Transparent Proxying**: Forwards requests to Anthropic and OpenAI APIs without modification
- **Multiple Named Endpoints**: Configure different proxy instances (dev, prod, staging, etc.)
- **TOML Configuration**: Easy-to-read configuration file format
- **Per-Endpoint Proxy Support**: Each endpoint can use different proxy servers
- **Flexible Target URLs**: Configure different target base URLs per endpoint
- **Environment Variable Support**: Override config file path with `CONFIG_PATH`

## Project Structure

```
├── Cargo.toml                  # Project dependencies and metadata
├── config.toml                 # Main configuration file
├── config.example.toml         # Example configuration file
├── Justfile                    # Command automation with just
├── Dockerfile                  # Docker image definition
├── docker-compose.yml          # Docker orchestration
├── src/                        # Source code
│   ├── main.rs                 # Entry point
│   ├── lib.rs                  # Library exports
│   ├── config.rs               # Configuration management
│   └── proxy.rs                # Core proxy logic
├── tests/                      # Integration and E2E tests
├── examples/                   # Deployment examples and scripts
├── README.md                   # Project documentation
├── CLAUDE.md                   # Guidelines for Claude Code
├── AGENTS.md                   # Agent guidelines
└── .copilot-instructions.md    # GitHub Copilot instructions
```

## Technology Stack

- **Language**: Rust 2021 edition
- **Web Framework**: Axum
- **HTTP Client**: reqwest
- **Configuration**: TOML format with serde
- **Logging**: tracing and tracing-subscriber
- **Build Tool**: Cargo
- **Task Runner**: Just
- **Containerization**: Docker

## Core Components

### 1. Main Application (src/main.rs)
- Entry point that loads configuration and starts the Axum server
- Sets up path-based routing: `/:prefix/v1/*path`
- Handles incoming requests through the proxy handler

### 2. Configuration Management (src/config.rs)
- `Config`: Main configuration structure
- `ServerConfig`: Server-level settings (port, default target base)
- `EndpointConfig`: Per-endpoint settings (proxy URL, target base override)
- Loads configuration from TOML files with fallback to defaults

### 3. Proxy Service (src/proxy.rs)
- `ProxyService`: Manages multiple HTTP clients with different proxy configurations
- Creates one client per endpoint with optional proxy settings
- Extracts endpoint name from URL path to select appropriate client
- Transparently forwards requests and responses between client and target API

## Request Flow

1. Client sends request to `http://localhost:8811/{endpoint_name}/v1/{api_path}`
2. Server extracts `endpoint_name` from path
3. ProxyService looks up endpoint configuration
4. Selects appropriate HTTP client (with or without proxy)
5. Forwards request to target API (Anthropic/OpenAI)
6. Returns response transparently to client

## Configuration

The proxy uses a TOML configuration file to define endpoints and their settings.

### Basic Configuration
```toml
[server]
port = 8811
target_base = "https://api.anthropic.com"

[endpoints.anthropic_dev]
proxy_url = "http://localhost:8080"
target_base = "https://api.anthropic.com"

[endpoints.openai_prod]
proxy_url = "http://proxy.company.com:3129"
target_base = "https://api.openai.com/v1"
```

### Configuration Options
- **Server Section**: 
  - `port`: Port to listen on (default: 8811)
  - `target_base`: Default target base URL for all endpoints
- **Endpoint Sections**: 
  - `proxy_url`: Proxy server URL for this endpoint (optional)
  - `target_base`: Target API base URL for this endpoint (optional)

## Development Commands

### Building and Running
```bash
# Build development version
cargo build
just build

# Build release version
cargo build --release
just build-release

# Run the proxy
cargo run
just run

# Run with custom port
just run-port 8080
```

### Testing
```bash
# Run all tests
cargo test
just test

# Run specific test suites
just test-unit        # Unit tests only
just test-integration # Integration tests
just test-e2e         # E2E tests
just test-verbose     # Tests with verbose output
```

### Code Quality
```bash
# Format code
cargo fmt
just fmt

# Check formatting
cargo fmt --check
just fmt-check

# Lint code
cargo clippy
just lint

# Lint with all features and warnings
just lint-all
```

### Project Management
```bash
# Clean build artifacts
cargo clean
just clean

# Check for outdated dependencies
cargo outdated
just outdated

# Update dependencies
cargo update
just update

# Generate documentation
cargo doc --no-deps
just docs

# Open documentation in browser
cargo doc --no-deps --open
just docs-open
```

## Deployment

### Docker
```bash
# Build and run Docker container
docker build -t anthropic-http-proxy .
docker run -p 8811:8811 anthropic-http-proxy

# Using just
just docker-build
just docker-run

# Using docker-compose
docker-compose up -d
```

### System Startup

#### Linux (systemd)
```bash
# Install the service
sudo ./examples/install-linux.sh

# Service management
sudo systemctl status anthropic-http-proxy
sudo systemctl start/stop/restart anthropic-http-proxy
sudo journalctl -u anthropic-http-proxy -f
```

#### macOS (launchd)
```bash
# Install the service
sudo ./examples/install-macos.sh

# Service management
sudo launchctl list | grep anthropic
sudo launchctl start/stop com.anthropic.http-proxy
tail -f /var/log/anthropic-http-proxy/output.log
```

#### Windows (NSSM)
```cmd
# Install the service
examples\install-windows.bat

# Service management
net start/stop anthropic-http-proxy
sc query anthropic-http-proxy
type C:\opt\anthropic-http-proxy\logs\output.log
```

## Environment Variables

- `CONFIG_PATH`: Path to configuration file (default: `config.toml`)
- `PORT`: Server port (overrides config file, default: 8811)

## Testing Strategy

- **Unit tests**: In-module tests for individual functions (config parsing, path extraction)
- **Integration tests**: Located in `tests/integration_tests.rs`
- **E2E tests**: Located in `tests/e2e_tests.rs`

## Code Style and Conventions

### Rust Guidelines
- **Edition**: Rust 2021
- **Naming**: snake_case for variables/functions, PascalCase for types/structs
- **Imports**: Group std, external, and local imports with blank lines
- **Error Handling**: Use Result<T, E> and ? operator, avoid unwrap() in production
- **Async**: Use tokio runtime with .await syntax

### Development Workflow
1. Follow the existing code style and patterns
2. Add tests for new functionality
3. Update documentation as needed
4. Ensure all tests pass and code lints cleanly
5. Use appropriate error handling and logging

## Common Patterns

- Use `tracing` for structured logging with appropriate levels
- Implement proper error types with `thiserror` or similar
- Use `serde` for serialization/deserialization of config and data
- Follow async/await patterns for network operations
- Use proper lifetime annotations when needed