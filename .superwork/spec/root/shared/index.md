# Repository-Level Guidelines

## Scope

Use this guide for root configuration, CI, scripts, assets, and changes spanning package groups.

## Repository Contracts

- Treat `Cargo.toml` and `Cargo.lock` as the workspace dependency contract.
- Keep generated GitHub workflows synchronized through `cargo xtask workflows`.
- Keep scripts repository-relative and preserve macOS, Linux, and Windows behavior where applicable.
- Store runtime assets under `assets/`; load them through established asset APIs.
- Follow repository-local `AGENTS.md` files for narrower directory rules.

## Verification

- Run `cargo fmt --all -- --check` after Rust changes.
- Run `./script/clippy` for workspace-wide Rust or dependency changes.
- Run `cargo nextest run --workspace --no-fail-fast --no-tests=warn` for broad behavior changes.
- Run the specific script or `cargo xtask` test when changing automation.

## Related Guides

- [Code reuse](../../guides/code-reuse-thinking-guide.md)
- [Cross-crate contracts](../../guides/cross-layer-thinking-guide.md)
- [Cross-platform changes](../../guides/cross-platform-thinking-guide.md)
