# Cross-Crate Contract Guide

## Goal

Make ownership, async execution, and data contracts explicit across crates.

## Checklist

- Trace a change from the entry point through model, service, and GPUI view consumers.
- Keep blocking work and network I/O off the foreground executor and render paths.
- Define cancellation, error propagation, offline behavior, and stale-data behavior.
- Update all direct consumers when intentionally changing a shared API or serialized format.
- Add focused tests at the contract boundary instead of duplicating implementation tests.
