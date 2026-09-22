# Frontend Crate Guidelines

## Scope

Use this guide for GPUI views, editor surfaces, panels, pickers, UI components, and input handling under `crates/`.

## Contracts

- Reuse components and styling patterns from `crates/ui` before adding feature-local variants.
- Keep `render` paths free of blocking work, avoid repeated allocation, and preserve stable element identity.
- Perform model mutation through GPUI context APIs and keep subscriptions owned for their required lifetime.
- Preserve keyboard, focus, accessibility, theme, and platform behavior for interactive controls.
- Keep multiline settings editors and their focus-out subscriptions in stable keyed state; cover pointer focus, text input, and blur-save behavior with an interaction test.
- Mark optional GUI settings that intentionally lack a raw default so debug builds render their control instead of the `NO DEFAULT` placeholder.
- Add visual or interaction tests only at stable user-visible boundaries.

## Verification

- Run `cargo fmt --all -- --check`.
- Run `./script/clippy -p <crate>`.
- Run `cargo nextest run -p <crate> --no-fail-fast --no-tests=warn`.
- Exercise the affected interaction when automated coverage cannot verify rendering or focus behavior.

## Related Guides

- [Cross-crate contracts](../../guides/cross-layer-thinking-guide.md)
- [Cross-platform changes](../../guides/cross-platform-thinking-guide.md)
