# Cross-Platform Guide

## Goal

Preserve macOS, Linux, and Windows behavior where the affected crate is shared.

## Checklist

- Check existing `cfg` gates and platform crates before adding platform-specific behavior.
- Keep common policy outside platform implementations; isolate native APIs behind existing traits.
- Use repository-relative paths and `python3` for Python commands.
- Provide PowerShell equivalents when adding required cross-platform shell workflows.
- Verify platform-specific changes with the matching CI command or targeted build when available.
