# Service Crate Guidelines

## Scope

Use this guide for networking, persistence, RPC, databases, background services, and server-facing code under `crates/`.

## Contracts

- Keep network and disk work asynchronous and off foreground or render executors.
- Bound queues, retries, buffers, and concurrency to protect memory use and tail latency.
- Preserve cancellation and propagate actionable errors across task and RPC boundaries.
- Treat protobuf, RPC, database, and persisted state changes as cross-consumer contract changes.
- Avoid extra serialization and copies on latency-sensitive paths; measure when impact is uncertain.

## Verification

- Run `cargo fmt --all -- --check`.
- Run `./script/clippy -p <crate>`.
- Run `cargo nextest run -p <crate> --no-fail-fast --no-tests=warn`.
- Run integration tests requiring PostgreSQL or external services in their documented environment.

## Related Guides

- [Cross-crate contracts](../../guides/cross-layer-thinking-guide.md)
- [Cross-platform changes](../../guides/cross-platform-thinking-guide.md)
