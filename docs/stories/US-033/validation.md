# Validation

## Proof Strategy

Prove v0.5.0 is releasable locally and then verify the public GitHub release
with Harness release verification.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Rust test suite. |
| Integration | Schema verifiers, installer syntax, release build. |
| E2E | Public release verify downloads assets, checks SHA256, version, and smoke. |
| Platform | GitHub workflow builds 5 platform binaries and 5 SHA256 assets. |
| Performance | Not applicable. |
| Logs/Audit | Detailed trace records release proof and installer payload review. |

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-friction-taxonomy.py
python scripts/verify-mcp-artifact-contracts.py
cargo build --package harness-cli --release
harness-cli release verify --version 0.5.0 --story US-033
harness-cli context --story US-033
harness-cli arch-check --story US-033
harness-cli story verify US-033
```

## Acceptance Criteria

- CLI version is `0.5.0`.
- Installer release pin is `harness-cli-v0.5.0`.
- Installer payload includes v0.4/v0.5 schema and decision contracts.
- Release notes exist.
- Local tests pass.
- Public GitHub release exists.
- Release has 10 assets.
- `release verify --version 0.5.0` passes.
- Backlog #3 is closed if installer payload drift is resolved.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#11`.
- Intake recorded: `#22`.
- Story recorded: `US-033`, lane `high-risk`, release proof required.
- CLI version bumped to `0.5.0`.
- `scripts/harness-cli-release-tag` updated to `harness-cli-v0.5.0`.
- `scripts/README.md` release verify example updated to `0.5.0`.
- `RELEASE_NOTES_v0.5.0.md` added.
- Installer payload now includes:
  - `docs/decisions/0010-mcp-artifact-contracts.md`;
  - `docs/decisions/0011-harness-friction-taxonomy.md`;
  - v0.4 MCP schemas;
  - v0.5 friction schema;
  - `scripts/schema/007-friction-events.sql`.
- Backlog `#3` closed as implemented for installer payload drift.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 46 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `scripts/install-harness.ps1` syntax check pass.
- `scripts/install-harness.sh` syntax check pass.
- `cargo build --package harness-cli --release` pass.
- `target/release/harness-cli.exe --version` reports `harness-cli 0.5.0`.
- Local Windows release package built:
  `dist/harness-cli-windows-x64.exe` and `.sha256`.
- Packaged Windows binary reports `harness-cli 0.5.0`.
