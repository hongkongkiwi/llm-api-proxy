# Build stage
FROM rust:1.70-slim AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs for dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r anthropic-proxy && useradd -r -g anthropic-proxy anthropic-proxy

# Create directories
RUN mkdir -p /opt/anthropic-http-proxy /etc/anthropic-http-proxy /var/log/anthropic-http-proxy && \
    chown -R anthropic-proxy:anthropic-proxy /opt/anthropic-http-proxy /etc/anthropic-http-proxy /var/log/anthropic-http-proxy

# Copy binary from builder
COPY --from=builder /app/target/release/anthropic-http-proxy /opt/anthropic-http-proxy/

# Copy example config
COPY config.example.toml /etc/anthropic-http-proxy/config.example.toml

# Switch to non-root user
USER anthropic-proxy

# Set working directory
WORKDIR /opt/anthropic-http-proxy

# Expose port
EXPOSE 8811

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8811/health || exit 1

# Default command
CMD ["/opt/anthropic-http-proxy/anthropic-http-proxy"]