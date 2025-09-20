# Agent Guidelines for anthropic-http-proxy

## Build/Lint/Test Commands
- Build: `cargo build` or `just build`
- Test single file: `cargo test --test <test_file>` (e.g., `cargo test --test integration_tests`)
- Test single function: `cargo test <test_name>`
- Lint: `cargo clippy` or `just lint`
- Format: `cargo fmt` or `just fmt`
- Run: `cargo run` or `just run`

## Code Style Guidelines
- **Language**: Rust 2021 edition
- **Imports**: Group std, external, and local imports with blank lines
- **Formatting**: Use rustfmt (cargo fmt)
- **Types**: Explicit types for function signatures, infer where obvious
- **Naming**: snake_case for variables/functions, PascalCase for types/structs
- **Error Handling**: Use Result<T, E> and ? operator, avoid unwrap() in production
- **Async**: Use tokio runtime, .await for async operations
- **Testing**: Unit tests in #[cfg(test)] modules, integration tests in tests/ directory