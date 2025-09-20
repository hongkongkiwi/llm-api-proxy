# Repository Guidelines

## Project Structure & Module Organization
- Core runtime sits in `src/`: `main.rs` boots the Axum server, `proxy.rs` applies routing logic, and `config.rs` / `lib.rs` expose shared types.
- Configuration templates live at the repo root—copy `config.example.toml` to `config.toml`; release and Docker helpers reside in `examples/` and `docker-compose.yml`.
- Async integration and e2e suites live in `tests/`; keep fixtures beside the scenarios they support, and lean on the `Justfile` for repeatable workflows.

## Build, Test, and Development Commands
- `cargo build` (`just build`) produces debug binaries; `cargo build --release` or `just build-release` outputs optimized artifacts.
- `cargo run`, `just run`, or `just run-port 8080` start the proxy against the active config—prefer env overrides (`CONFIG_PATH`, `ENDPOINT_*`) over code edits.
- `cargo test` / `just test` run the full suite, while `cargo test --test integration_tests` or `just test-e2e` target specific flows; always finish with `cargo fmt` (`just fmt`) and `cargo clippy --all-targets --all-features -- -D warnings` (`just lint-all`).

## Coding Style & Naming Conventions
- Rust 2021 with rustfmt defaults (4-space indent, trailing commas); group imports by std, third-party, then crate modules separated with blank lines.
- Use `snake_case` for functions and fields, `PascalCase` for types, and factor reusable helpers into `lib.rs` to keep modules focused.
- Prefer `Result` + `?` for error handling, avoid `unwrap` outside tests, and document non-obvious behavior with short, targeted comments.

## Testing Guidelines
- Name tests for observable behavior (`test_proxy_with_path_prefix`) and keep setup helpers private to the module.
- Mock upstream APIs with lightweight Axum routers instead of live services; seed env-dependent values via scoped helpers.
- Add integration coverage in `tests/` plus unit tests in local `#[cfg(test)]` blocks, and note the exact `cargo test` command in your PR.

## Commit & Pull Request Guidelines
- Write imperative commits aligned with recent history (`Add`, `Update`, optional `docs:` scope) and keep each change focused.
- Call out configuration or API shifts in the summary so operators know what to roll forward.
- PRs should cover problem, solution, config/env changes, and test evidence (paste the command you ran); link issues and surface rollout risks early.

## Configuration & Environment Notes
- Start from `config.example.toml`, keep secrets out of git, and rely on env vars like `CONFIG_PATH` or `ENDPOINT_<NAME>_PROXY` for overrides.
- Default listener is port `8811`; `just docker-run` mirrors that mapping for container smoke tests—document any new env vars in `README.md`.
