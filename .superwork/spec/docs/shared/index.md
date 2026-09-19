# Documentation Guidelines

## Scope

Use this guide for user documentation, the mdBook theme, preprocessors, and reference generation under `docs/`.

## Contracts

- Follow `docs/AGENTS.md` before editing documentation.
- Keep links repository-relative where possible and preserve generated reference-file workflows.
- Keep theme changes accessible and verify responsive layout in the built documentation.
- Update documentation with user-visible behavior changes when the existing docs would become inaccurate.

## Verification

- Run the documented mdBook build or preview command for content and theme changes.
- Run `cargo test --workspace --doc --no-fail-fast` for Rust documentation changes.
- Check links and rendered output for modified pages.
