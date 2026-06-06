# Exec Plan

## Goal

Turn the manual HI-OS release audit into a deterministic governance command
that verifies the trusted distribution chain and emits durable evidence.

Design review status: accepted and implementation-ready.

## Scope

In scope:

- Add `harness-cli release verify --version <version>`.
- Read the canonical public origin from accepted policy or tracked config.
- Verify the expected release tag and platform asset set.
- Verify matching SHA256 assets.
- Download the current-platform CLI without authentication.
- Verify the downloaded checksum and CLI version.
- Run a non-destructive smoke command.
- Generate machine-readable release evidence.
- Define how release evidence participates in story governance.

Out of scope:

- Changing the v0.2 tag or its ten release assets.
- Publishing a new release.
- Changing the canonical public origin.
- Signing artifacts beyond the existing SHA256 contract.
- Implementing a real-time MCP adapter.

## Risk Classification

Risk flags:

- External systems.
- Public contracts.
- Security and distribution trust.
- Architecture.
- Weak proof.

Hard gates:

- Public release and download behavior.
- Validation and governance requirements.

## Work Phases

1. Discover the installer, workflow, and release asset contracts.
2. Finalize the command and evidence schema.
3. Define deterministic provider boundaries and test fixtures.
4. Implement the smallest current-platform verification slice.
5. Add success and failure coverage.
6. Generate real release evidence against the canonical public origin.
7. Integrate release evidence with story governance.
8. Record detailed trace and validation proof.

## Stop Conditions

Pause for human confirmation if:

- Decision `0008` would need to change.
- Verification would require GitHub credentials for public assets.
- The v0.2 tag or release assets would need mutation.
- Story governance requirements would be weakened.
- A cross-platform execution policy cannot be defined without expanding scope.
- The canonical origin resolved at runtime differs from decision `0008`.
- Network or GitHub availability cannot be distinguished from a trust failure.
- Evidence cannot be written to both the operational report and durable
  summary without partial-state ambiguity.
