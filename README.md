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
```

### Run

```bash
cargo run
# or with custom config
CONFIG_PATH=/path/to/config.toml cargo run
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
just lint
```

### Development Commands

```bash
# Run with hot reload (requires cargo-watch)
cargo watch -x run

# Run with custom port
cargo run -- --port 8080

# Using just
just run-port 8080
```

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