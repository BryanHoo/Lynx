# Lynx Engineering Guides

## Scope

Project-wide checks for changes that cross crate, platform, or ownership boundaries.

## Pre-Development Checklist

- Read applicable `AGENTS.md` files before changing files.
- Use `cargo metadata --no-deps` and dependency declarations to identify affected crates.
- Read the matching package/layer index selected by Superwork context.
- Keep latency, memory, rendering cost, and allocations visible in design decisions.

## Verification Checklist

- Run the narrowest crate-level test first, then broaden when a shared contract changes.
- Run `cargo fmt --all -- --check` for Rust changes.
- Run `./script/clippy -p <crate>` for a targeted crate or `./script/clippy` for workspace-wide changes.
- Run `cargo nextest run -p <crate> --no-fail-fast --no-tests=warn` for targeted tests.
- Run `cargo test --workspace --doc --no-fail-fast` when public Rust documentation changes.

## Update Triggers

- A repeated failure establishes a durable repository rule.
- A crate boundary or public contract changes.
- CI changes the required verification commands.
