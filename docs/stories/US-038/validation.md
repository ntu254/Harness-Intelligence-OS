# Validation

## Proof Strategy

Prove v0.6.0 is releasable locally, publish it publicly, then verify the public
release with Harness release verification and governance dashboard evidence.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Rust test suite. |
| Integration | Schema verifiers, installer syntax, release build. |
| E2E | Public release verify downloads assets, checks SHA256, version, and smoke. |
| Platform | GitHub workflow builds 5 platform binaries and 5 SHA256 assets. |
| Performance | Not applicable. |
| Logs/Audit | Detailed trace records release and dashboard proof. |

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-governance-report-schema.py
python scripts/verify-friction-taxonomy.py
python scripts/verify-mcp-artifact-contracts.py
cargo build --package harness-cli --release
harness-cli governance report --output .harness/reports/US-038-governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/US-038-governance-report.json
harness-cli governance dashboard --report .harness/reports/US-038-governance-report.json --output .harness/dashboard/US-038-index.html
harness-cli release verify --version 0.6.0 --story US-038
harness-cli context --story US-038
harness-cli arch-check --story US-038
harness-cli story verify US-038
```

## Acceptance Criteria

- CLI version is `0.6.0`.
- Installer release pin is `harness-cli-v0.6.0`.
- Installer payload includes v0.6 governance report docs, schema, decision, and
  verifier.
- Release notes exist.
- Local tests pass.
- Public GitHub release exists.
- Release has 10 assets.
- `release verify --version 0.6.0` passes.
- Governance report smoke passes.
- Governance dashboard smoke passes.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

Pending release validation.
