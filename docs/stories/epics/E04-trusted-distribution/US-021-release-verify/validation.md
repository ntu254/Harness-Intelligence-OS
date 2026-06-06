# Validation

## Proof Strategy

Separate pure release-contract rules from network and process boundaries.
Unit tests should use deterministic metadata and checksum fixtures. Integration
tests should use a local HTTP fixture or injectable release client. One explicit
release proof should audit the canonical public v0.2.0 release.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Version parsing, tag construction, platform asset matrix, missing/duplicate assets, checksum parsing, report result aggregation |
| Integration | Release missing, binary missing, checksum missing, unauthenticated download failure, hash mismatch, wrong CLI version, smoke failure, atomic report write |
| E2E | Verify public `harness-cli-v0.2.0` and produce passing evidence |
| Platform | Current-platform binary executes; unsupported platform fails clearly |
| Performance | Downloads are bounded to one platform binary and checksum |
| Logs/Audit | JSON report and detailed linked trace contain all required checks |

## Fixtures

- Complete ten-asset release metadata.
- Missing binary and missing checksum metadata.
- Duplicate asset names.
- Valid and invalid checksum files.
- Binary process fixtures for matching version, wrong version, and smoke fail.
- Temporary evidence output directory.

## Commands

Implementation-phase commands are expected to include:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
harness-cli release verify --version 0.2.0
harness-cli arch-check --story US-021
harness-cli story verify US-021
```

Do not configure the story verify command until the implementation and test
contract exist.

## Acceptance Evidence

Required before implementation can be marked complete:

- Canonical public origin is read from accepted policy or tracked config.
- Expected tag exists.
- Five platform binaries and five matching SHA256 assets exist.
- Current-platform binary download succeeds without authentication.
- SHA256 verification passes.
- Downloaded CLI reports the requested version.
- Smoke command passes.
- Evidence report is generated with `result: pass`.
- Story governance requires passing release evidence for installer or release
  distribution work.
- Detailed trace is linked to `US-021`.

Planning evidence:

- Decision `0008` is accepted.
- Public v0.2.0 release and ten immutable assets exist.
- US-020 public installer smoke proof passed.
- No release verification implementation was started in this planning step.
