# Repository-Level Guidelines

## Scope

Use this guide for root configuration, CI, scripts, assets, and changes spanning package groups.

## Repository Contracts

- Treat `Cargo.toml` and `Cargo.lock` as the workspace dependency contract.
- Keep GitHub workflows limited to automation that can run in the Lynx repository without Zed internal secrets or services.
- Keep scripts repository-relative and preserve macOS, Linux, and Windows behavior where applicable.
- Store runtime assets under `assets/`; load them through established asset APIs.
- Follow repository-local `AGENTS.md` files for narrower directory rules.

## Product Boundaries

- Keep project creation, restoration, persistence, terminals, and editor workflows local-only; do not add SSH, WSL, or remote-server development entry points, builds, packages, or release artifacts.
- Do not add Zed account, billing, organization, hosted-model, collaboration, or telemetry-upload services to Lynx.
- Route AI requests through user-configured provider APIs, gateways, local models, or external agents.
- Do not publish Zed company policies or point Lynx package metadata at Zed-operated support and release services.

## Verification

- Run `cargo fmt --all -- --check` after Rust changes.
- Run `./script/clippy` for workspace-wide Rust or dependency changes.
- Run `cargo nextest run --workspace --no-fail-fast --no-tests=warn` for broad behavior changes.
- Run the specific script or `cargo xtask` test when changing automation.
- Search changed automation, documentation, and package metadata for Zed-only domains and secret names after upstream cleanup.

## Related Guides

- [Code reuse](../../guides/code-reuse-thinking-guide.md)
- [Cross-crate contracts](../../guides/cross-layer-thinking-guide.md)
- [Cross-platform changes](../../guides/cross-platform-thinking-guide.md)
