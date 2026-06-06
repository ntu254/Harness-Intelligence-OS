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

- GitHub issue created: `#16`.
- Intake recorded: `#27`.
- Story recorded: `US-038`, lane `high-risk`, release proof required.
- CLI version bumped to `0.6.0`.
- `scripts/harness-cli-release-tag` updated to `harness-cli-v0.6.0`.
- `scripts/README.md` release verify example updated to `0.6.0`.
- `RELEASE_NOTES_v0.6.0.md` added.
- Installer payload now includes:
  - `docs/GOVERNANCE_REPORT.md`;
  - `docs/decisions/0012-governance-report-schema.md`;
  - `docs/schemas/governance-report.schema.json`;
  - `scripts/verify-governance-report-schema.py`.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 49 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `python scripts/verify-governance-report-schema.py` pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `scripts/install-harness.ps1` syntax check pass.
- `scripts/install-harness.sh` syntax check pass.
- `cargo build --package harness-cli --release` pass.
- `target/release/harness-cli.exe --version` reports `harness-cli 0.6.0`.
- Local Windows release package/checksum produced from the validated release
  binary:
  - `dist/harness-cli-windows-x64.exe`;
  - `dist/harness-cli-windows-x64.exe.sha256`.
- Packaged Windows binary reports `harness-cli 0.6.0`.
- Release prep commit pushed: `6345b47`.
- Annotated tag pushed: `harness-cli-v0.6.0`.
- GitHub release workflow `27075127128` passed:
  - Verify job passed;
  - 5 platform build jobs passed;
  - Publish GitHub Release job passed.
- Public release:
  `https://github.com/ntu254/Harness-Intelligence-OS/releases/tag/harness-cli-v0.6.0`.
- Public release assets: 10.
- `harness-cli release verify --version 0.6.0 --story US-038` pass:
  - release tag: `harness-cli-v0.6.0`;
  - origin: `ntu254/Harness-Intelligence-OS`;
  - platform: `windows-x64`;
  - assets checked: 10;
  - download: pass;
  - checksum: pass;
  - version: pass;
  - smoke install: pass.
- Release evidence file:
  `.harness/release/harness-cli-v0.6.0-release-verify.json`.
- `target/release/harness-cli.exe governance report --output .harness/reports/US-038-governance-report.json`
  pass.
- `python scripts/verify-governance-report-schema.py .harness/reports/US-038-governance-report.json`
  pass.
- `target/release/harness-cli.exe governance dashboard --report .harness/reports/US-038-governance-report.json --output .harness/dashboard/US-038-index.html`
  pass.
- Final governance dashboard after story gate:
  - stories: 20;
  - gate pass: 18;
  - maturity: `trusted (91)`.
- `harness-cli context --story US-038` pass.
- `harness-cli arch-check --story US-038` pass.
- Detailed trace recorded: `#36`, score `3/3`.
- `harness-cli story verify US-038` pass.
- Story governance gate pass.
- No SQLite migration was added.
- Older release tags and assets remain unchanged.
