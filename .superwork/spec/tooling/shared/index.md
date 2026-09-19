# Tooling Guidelines

## Scope

Use this guide for `tooling/xtask`, custom lints, compliance checks, and performance tooling.

## Contracts

- Keep generated artifacts deterministic and document the command that regenerates them.
- Use `cargo xtask workflows` for generated GitHub Actions workflows.
- Keep custom lints focused, actionable, and covered by positive and negative fixtures.
- Keep benchmark setup representative of production behavior and isolate measurement overhead.

## Verification

- Run the changed tooling crate's focused tests.
- Run `cargo test --package xtask -- workspace::` for workspace tooling changes.
- Run the lint UI suite when changing `tooling/lints`.
- Regenerate affected artifacts and verify a clean diff.
