# Shared Crate Guidelines

## Scope

Use this guide for core models, algorithms, language support, utilities, and other reusable crates under `crates/`.

## Contracts

- Place behavior in the crate that owns its data and lifecycle.
- Prefer borrowing and existing shared types on hot paths; clone only when ownership or task boundaries require it.
- Keep public APIs narrow and inspect reverse dependencies before changing them.
- Document complexity, allocation, caching, and invalidation assumptions for performance-sensitive code.
- Add benchmarks for changes where latency, throughput, or memory use is a defining contract.

## Verification

- Run `cargo fmt --all -- --check`.
- Run `./script/clippy -p <crate>`.
- Run `cargo nextest run -p <crate> --no-fail-fast --no-tests=warn`.
- Run the relevant Criterion or `cargo perf-test -p <crate>` benchmark for performance work.

## Related Guides

- [Code reuse](../../guides/code-reuse-thinking-guide.md)
- [Cross-crate contracts](../../guides/cross-layer-thinking-guide.md)
