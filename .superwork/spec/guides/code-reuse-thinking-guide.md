# Code Reuse Guide

## Goal

Keep reusable behavior at the narrowest stable crate boundary.

## Checklist

- Search `crates/`, `extensions/`, and `tooling/` before adding a new helper or dependency.
- Prefer an existing crate API when it already owns the relevant data or lifecycle.
- Keep feature-specific behavior in its owning crate until multiple real consumers need it.
- Avoid dependency cycles and broad utility crates; verify reverse dependencies before moving APIs.
