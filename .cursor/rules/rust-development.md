# Rust Development Rules for anthropic-http-proxy

## Core Guidelines
- **Language**: Rust 2021 edition
- **Runtime**: tokio for async operations
- **Build**: Cargo with justfile shortcuts

## Code Style
- Variables/functions: `snake_case`
- Types/structs: `PascalCase`
- Imports: std → external → local (blank line separated)
- Error handling: `Result<T, E>` + `?` operator
- No `unwrap()` in production code

## Project Structure
```
src/           # Main application code
├── main.rs    # Application entry point
├── lib.rs     # Library exports
├── config.rs  # Configuration management
└── proxy.rs   # HTTP proxy logic

tests/         # Integration tests
examples/      # Installation scripts and examples
```

## Key Dependencies
- `tokio`: Async runtime
- `hyper`: HTTP server/client
- `tracing`: Structured logging
- `serde`: Serialization
- `toml`: Config parsing

## Development Commands
- Build: `cargo build` or `just build`
- Test: `cargo test` or `cargo test --test <file>`
- Lint: `cargo clippy` or `just lint`
- Format: `cargo fmt` or `just fmt`
- Run: `cargo run` or `just run`

## Testing
- Unit tests: `#[cfg(test)]` modules in source files
- Integration tests: `tests/` directory
- Test both success and error cases
- Use descriptive test names

## Security
- Never commit secrets/API keys
- Use environment variables for sensitive config
- Validate all external input
- Follow Rust security best practices

## Performance
- Use async/await for I/O
- Minimize allocations in hot paths
- Profile and optimize bottlenecks