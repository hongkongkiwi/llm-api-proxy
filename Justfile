# Justfile for anthropic-http-proxy

# Build the project
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run the project
run:
    cargo run

# Run the project with a specific port
run-port port='8811':
    cargo run --bin anthropic-http-proxy -- --port {{port}}

# Run all tests
test:
    cargo test

# Run unit tests only
test-unit:
    cargo test --lib

# Run integration tests only
test-integration:
    cargo test --test integration_tests

# Run E2E tests only
test-e2e:
    cargo test --test e2e_tests

# Run tests with verbose output
test-verbose:
    cargo test -- --nocapture

# Check code formatting
fmt:
    cargo fmt

# Check code formatting and apply changes
fmt-check:
    cargo fmt --check

# Lint the code
lint:
    cargo clippy

# Lint the code with all features
lint-all:
    cargo clippy --all-targets --all-features -- -D warnings

# Check for outdated dependencies
outdated:
    cargo outdated

# Update dependencies
update:
    cargo update

# Clean build artifacts
clean:
    cargo clean

# Generate documentation
docs:
    cargo doc --no-deps

# Open documentation in browser
docs-open:
    cargo doc --no-deps --open

# Run the project with environment variables
run-dev:
    ANTHROPIC_API_BASE=http://localhost:8000 cargo run

# Run the project with custom proxy configuration
run-with-proxy proxy_url='http://localhost:8080':
    ENDPOINT_DEFAULT_PROXY={{proxy_url}} cargo run

# Build and run Docker container (if Dockerfile exists)
docker-build:
    docker build -t anthropic-http-proxy .

# Run Docker container
docker-run:
    docker run -p 8811:8811 anthropic-http-proxy

# Show help for just commands
help:
    @just --list

# Version management
# Bump patch version (1.0.0 -> 1.0.1)
bump-patch:
    cargo bump patch
    @echo "Version bumped to $(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "anthropic-http-proxy") | .version')"

# Bump minor version (1.0.0 -> 1.1.0)
bump-minor:
    cargo bump minor
    @echo "Version bumped to $(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "anthropic-http-proxy") | .version')"

# Bump major version (1.0.0 -> 2.0.0)
bump-major:
    cargo bump major
    @echo "Version bumped to $(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "anthropic-http-proxy") | .version')"

# Set specific version
bump-version version:
    cargo bump {{version}}
    @echo "Version set to $(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "anthropic-http-proxy") | .version')"

# Create release tag and push
release:
    #!/usr/bin/env bash
    set -e
    VERSION=$(cargo metadata --format-version=1 | jq -r '.packages[] | select(.name == "anthropic-http-proxy") | .version')
    echo "Creating release for version $VERSION"
    git add Cargo.toml
    git commit -m "Release version $VERSION"
    git tag -a "v$VERSION" -m "Release version $VERSION"
    git push origin main
    git push origin "v$VERSION"

# Build release binaries for all platforms
build-release-binaries:
    ./examples/build-release.sh

# Default command
default:
    @just help