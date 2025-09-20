# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a transparent HTTP proxy for LLM APIs (Anthropic, OpenAI) written in Rust. It uses single-port, path-based routing to direct requests to different proxy configurations.

## Development Commands

### Build & Run
```bash
# Build development version
cargo build
just build

# Build release version
cargo build --release
just build-release

# Run the proxy (default port 8811)
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
just test-unit        # Unit tests only (--lib)
just test-integration # Integration tests (--test integration_tests)
just test-e2e        # E2E tests (--test e2e_tests)
just test-verbose    # Tests with verbose output (-- --nocapture)
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

# Check and update dependencies
just outdated    # Check for outdated dependencies
just update      # Update dependencies
just clean       # Clean build artifacts
```

### Docker
```bash
# Build Docker image
just docker-build

# Run Docker container
just docker-run
```

### Development
```bash
# Development workflow
just run-dev                    # Run with ANTHROPIC_API_BASE env var
just run-with-proxy URL         # Run with custom proxy URL
just docs                       # Generate documentation
just docs-open                  # Open docs in browser
```

## Architecture

### Core Components

1. **main.rs**: Entry point and Axum server setup
   - Loads configuration from `config.toml` or `CONFIG_PATH` env var
   - Sets up path-based routing: `/:prefix/v1/*path`
   - Creates proxy handler for incoming requests

2. **config.rs**: Configuration management
   - `Config`: Main config structure with server settings and endpoint configurations
   - `ServerConfig`: Port and default target base URL
   - `EndpointConfig`: Per-endpoint proxy URL and optional target base override
   - Loads from TOML files with fallback to defaults

3. **proxy.rs**: Core proxy logic
   - `ProxyService`: Main service that creates and manages HTTP clients for each endpoint
   - `new()`: Creates service with default target base from config
   - `new_with_config()`: Creates service with specific configuration
   - `handle_request()`: Routes requests to appropriate client based on endpoint name
   - Each endpoint gets independent reqwest::Client with optional proxy settings
   - Transparently forwards requests/responses without modification

### Request Flow

1. Client sends request to `http://localhost:8811/{endpoint_name}/v1/{api_path}`
2. Server extracts `endpoint_name` from path
3. ProxyService looks up endpoint configuration
4. Selects appropriate HTTP client (with or without proxy)
5. Forwards request to target API (Anthropic/OpenAI)
6. Returns response transparently to client

### Key Design Decisions

- **Single Port Operation**: All endpoints served through one port (8811 by default)
- **Path-Based Routing**: First path segment determines which proxy configuration to use
- **Per-Request Config Loading**: Configuration is reloaded for each request (allows runtime config changes)
- **Transparent Proxying**: No modification of request/response content
- **Multiple HTTP Clients**: Each endpoint gets its own reqwest::Client with independent proxy settings

## Configuration

Configuration is managed through `config.toml`:
- Server settings: port and default target base URL
- Endpoints: HashMap of named configurations with optional proxy URLs and target base overrides
- Environment variable `CONFIG_PATH` can override config file location
- Environment variable `PORT` can override the server port
- Environment variables: `ENDPOINT_{NAME}_PROXY` can set proxy URLs per endpoint

## Testing Strategy

- **Unit tests**: In-module tests for individual functions (config parsing, path extraction)
- **Integration tests**: Located in `tests/integration_tests.rs`
  - Tests ProxyService creation and basic request handling
  - Tests path prefix routing and error scenarios
  - Tests environment variable proxy configuration
- **E2E tests**: Located in `tests/e2e_tests.rs`
  - Tests full proxy request flow with mock target servers
  - Tests custom port functionality
  - Uses actual HTTP requests to verify end-to-end behavior