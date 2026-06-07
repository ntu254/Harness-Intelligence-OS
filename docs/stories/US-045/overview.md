# US-045: HI-OS v0.7.0 Release Hardening

## Status

Implemented / High-Risk

## Issue

- GitHub issue: #26
- Milestone: HI-OS v0.7.0 Adoption Ready

## Goal

Publish HI-OS v0.7.0 with trusted native CLI assets and the production-clean
distribution payload introduced by US-048.

## In Scope

- Confirm all v0.7 content stories are complete.
- Bump the CLI and installer release pin to 0.7.0.
- Add v0.7.0 release notes.
- Extend release verification and CI to require the production ZIP and SHA256.
- Publish five native binaries, five binary SHA256 files, the production ZIP,
  and the ZIP SHA256.
- Verify the public release and public installer.
- Export final governance report/dashboard evidence.
- Close issue #26 and the v0.7 milestone.

## Out Of Scope

- New product features.
- New SQLite migrations.
- MCP provider contract changes.
- Rewriting older release tags or assets.
