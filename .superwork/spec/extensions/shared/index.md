# Extension Guidelines

## Scope

Use this guide for extension manifests, language assets, extension host APIs, and test extensions under `extensions/`.

## Contracts

- Keep extension manifests and bundled language assets synchronized with their Rust implementation.
- Treat extension API changes as public contracts and update all in-repository consumers together.
- Preserve sandbox and host boundaries; avoid unnecessary data transfer across them.
- Keep generated or fetched language-server metadata deterministic.

## Verification

- Run `cargo fmt -p <extension-package> -- --check`.
- Run `cargo clippy -p <extension-package> --release --all-features -- --deny warnings`.
- Run `cargo nextest run -p <extension-package> --no-fail-fast --no-tests=warn`.
