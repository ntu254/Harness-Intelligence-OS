# HI-OS v0.2.0: Blocking Governance Gate

This release upgrades Harness Intelligence OS from context generation to
enforceable governance.

## Added

- Configurable architecture checks via `harness-architecture.toml`.
- Blocking `story verify` governance gate.
- Structured JSON intake parsing.
- Segment-safe risk detection.
- Schema, installer, decision, story packet, and documentation updates.

## Changed

- CLI version bumped to `0.2.0`.
- Installer release pin bumped to `harness-cli-v0.2.0`.

## Verified

- 25 tests passed.
- `cargo fmt` passed.
- `cargo clippy` passed.
- Release build passed.
- Installer checks passed.
- US-019 architecture, mechanical verification, governance gate, and detailed
  trace passed.

## Release Assets

The release workflow publishes native CLI binaries and SHA256 files for:

- macOS arm64
- macOS x64
- Linux arm64
- Linux x64
- Windows x64
