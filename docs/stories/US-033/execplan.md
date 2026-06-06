# Exec Plan

## Goal

Publish and verify HI-OS v0.5.0 with trusted distribution evidence.

## Scope

In scope:

- Bump `harness-cli` version to `0.5.0`.
- Update release tag pin.
- Add v0.5.0 release notes.
- Review and update installer payload coverage.
- Run local validation.
- Build release binary.
- Publish GitHub release assets.
- Verify public release with `release verify`.
- Record Detailed trace and governance evidence.

Out of scope:

- v0.6 dashboard implementation.
- NotebookLM account/session setup.
- Changing v0.4.0 tag or assets.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Public release.
- Installer behavior.
- Trusted distribution evidence.
- Governance release proof.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Bump version and installer pin.
3. Fix installer payload drift.
4. Add release notes.
5. Run local validation.
6. Tag and publish release.
7. Verify public release with Harness.
8. Record trace and close issue.

## Stop Conditions

Pause if:

- Release assets cannot be produced for all five platforms.
- SHA256 verification fails.
- Version smoke does not report `0.5.0`.
- Governance gate cannot obtain release proof.
