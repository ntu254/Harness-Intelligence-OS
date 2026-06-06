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

## Failure Matrix

| Failure | Result | Required evidence |
| --- | --- | --- |
| Release tag does not exist | fail | Requested tag and HTTP/API result |
| Platform binary is missing | fail | Expected and observed asset names |
| Matching SHA256 asset is missing | fail | Expected checksum asset name |
| Checksum mismatch | fail | Expected and actual SHA256 |
| Public download is rejected | fail | URL and HTTP status when authoritative |
| Network or GitHub is unavailable | inconclusive | Transport error and attempted URL |
| Downloaded binary reports wrong version | fail | Expected and observed version |
| Smoke command exits non-zero | fail | Command, exit code, output summary |
| Runtime config origin differs from decision `0008` | fail | Canonical and configured origins |
| Report write or SQLite summary persistence fails | fail | Target path or database error |

`inconclusive` exits non-zero and never satisfies story governance.

## Fixtures

- Complete ten-asset release metadata.
- Missing binary and missing checksum metadata.
- Duplicate asset names.
- Valid and invalid checksum files.
- Binary process fixtures for matching version, wrong version, and smoke fail.
- Canonical-origin mismatch fixture.
- Transport timeout and unavailable-host fixture.
- Temporary evidence output directory.

## Commands

Validation commands:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
harness-cli release verify --version 0.2.0
harness-cli arch-check --story US-021
harness-cli story verify US-021
```

The story verification command is configured after the implementation and test
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
  distribution work marked `release_proof_required`.
- Detailed trace is linked to `US-021`.

## Implementation Evidence

- `cargo test --workspace`: 27 tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo fmt --all -- --check`: passed after formatting.
- Public release verification passed for
  `ntu254/Harness-Intelligence-OS`, tag `harness-cli-v0.2.0`.
- Ten release assets were discovered: five native binaries and five matching
  SHA256 files.
- The Windows x64 binary and checksum downloaded without authentication.
- Expected and actual SHA256 matched:
  `af2cac539f1b8571451c2b79224afa539bcb2da886fd7f87639f6e0e90c00ee8`.
- The downloaded binary reported `harness-cli 0.2.0`.
- The smoke command `arch-check --help` passed.
- Operational evidence was written to
  `.harness/release/harness-cli-v0.2.0-release-verify.json`.
- SQLite summary result is `pass` and is linked to `US-021`.
- PowerShell and Bash installer syntax checks passed.
- A clean Windows installer smoke created schema version 5 and propagated the
  release policy, migration, and decisions.
- Architecture check passed for `US-021`.
- Detailed implementation trace `#10` achieved the required High-Risk tier
  `3/3`.
- `story verify US-021` passed the mechanical checks and governance gate.

## Design Review

- Command contract accepted:
  `release verify --version <version> [--origin] [--platform] [--output] [--story]`.
- Default origin comes from tracked `harness-release.toml`.
- Option C accepted: full JSON report plus durable SQLite summary.
- Story governance uses an explicit durable `release_proof_required` flag.
- Network/GitHub unavailability is `inconclusive`, not `pass`.
- Story moved to In Progress only when implementation began.

Planning evidence:

- Decision `0008` is accepted.
- Public v0.2.0 release and ten immutable assets exist.
- US-020 public installer smoke proof passed.
- No release verification implementation was started in this planning step.
