# Validation

## Proof Strategy

Prove scanner precision, gate completeness, migration behavior, CLI help, and
release-quality Rust checks.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Segment matching, false-positive avoidance, missing gate evidence, complete gate |
| Integration | Schema migrations and story-linked architecture results |
| E2E | CLI smoke for `arch-check` and `story verify` |
| Platform | Windows repo-local binary |
| Performance | Not required for the MVP scanner |
| Logs/Audit | Durable story result and linked trace |

## Fixtures

- Temporary layered source trees.
- Temporary SQLite databases migrated through version 4.
- High-risk story with complete and incomplete evidence.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Acceptance Evidence

- `cargo fmt --all -- --check`
- `cargo test --workspace` passed with 25 tests.
- `cargo clippy --workspace -- -D warnings`
- `cargo build --release --workspace`
- `bash -n scripts/install-harness.sh`
- PowerShell parser accepted `scripts/install-harness.ps1`.
- PowerShell installer dry run included migrations `003` and `004`,
  `harness-architecture.toml`, and the HI-OS v0.2 decision.
- Repo-local Windows CLI reported version `0.2.0`.
- `arch-check --story US-019` scanned two configured files and passed.
