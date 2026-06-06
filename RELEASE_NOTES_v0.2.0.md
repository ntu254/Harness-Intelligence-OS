# HI-OS v0.2.0: Blocking Governance Gate

This release upgrades Harness Intelligence OS from context generation to
enforceable governance.

## Release Status

HI-OS v0.2.0 is released privately with verified release artifacts.

It is not public ship-ready because the canonical public release origin has not
been established. The public installer still points to an upstream repository
that does not host the v0.2.0 release.

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

## Next Milestone

HI-OS v0.3.0: Trusted Distribution & Evidence Trail will first establish the
canonical public release origin. It will then verify the complete trusted
distribution chain:

```text
publish -> download -> checksum -> version -> smoke install -> evidence trail
```

Its release verification must confirm the expected tag and platform assets,
SHA256 assets, installer pin and origin, downloaded checksum, CLI version,
smoke installation, and generated governance evidence.
