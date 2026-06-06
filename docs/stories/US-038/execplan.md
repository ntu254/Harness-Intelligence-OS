# Exec Plan

## Goal

Publish and verify HI-OS v0.6.0 with trusted distribution and governance
dashboard evidence.

## Scope

In scope:

- Bump `harness-cli` version to `0.6.0`.
- Update installer release pin to `harness-cli-v0.6.0`.
- Update installer payload for v0.6 docs/schema/verifier.
- Add `RELEASE_NOTES_v0.6.0.md`.
- Run local validation.
- Tag and publish the GitHub release.
- Run `harness-cli release verify --version 0.6.0 --story US-038`.
- Generate governance report and dashboard evidence.
- Record Detailed trace and close issue.

Out of scope:

- New dashboard features.
- SQLite migration.
- Changing older release assets.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Public release.
- Trusted distribution chain.
- Installer payload.
- Governance dashboard evidence.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Bump version and installer pin.
3. Update installer payload and release notes.
4. Run local validation and package smoke.
5. Commit, tag, and push release prep.
6. Wait for GitHub release workflow.
7. Run release verify and dashboard evidence.
8. Record trace, story gate, and final evidence commit.

## Stop Conditions

Pause if:

- Release workflow fails.
- Public assets are incomplete.
- SHA256/version/smoke verification fails.
- Installer payload syntax fails.
