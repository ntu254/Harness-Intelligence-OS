# Validation

## Proof Strategy

Prove `harness-cli governance report` generates schema-valid JSON and remains
read-only with respect to Harness governance state.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Report structs serialize to the US-034 shape. |
| Integration | Repository generator reads story, gate, validation, release, friction, and backlog state. |
| E2E | CLI writes `.harness/reports/US-035-governance-report.json` and schema verifier accepts it. |
| Platform | Windows release binary smoke for command output. |
| Performance | Report generation is local SQLite reads only. |
| Logs/Audit | Detailed trace records read-only behavior. |

## Required Failure / Guardrail Cases

- Generated report fails if schema validation fails.
- Report generation does not mutate `story.gate_result`.
- Story gate `inconclusive` is not emitted.
- Release inconclusive remains distinct from pass.
- Missing evidence appears in story rows rather than being hidden.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-governance-report-schema.py
cargo build --package harness-cli --release
harness-cli governance report --output .harness/reports/US-035-governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/US-035-governance-report.json
harness-cli context --story US-035
harness-cli arch-check --story US-035
harness-cli story verify US-035
```

## Acceptance Criteria

- `harness-cli governance report` exists.
- Default report output is under `.harness/reports/`.
- Custom `--output` is supported.
- Generated report matches the US-034 schema.
- Report includes repository, story, gate, validation, release, friction, and
  story-row summaries.
- Report rows include missing evidence when present.
- Report generation is read-only for Harness governance state.
- No SQLite migration is added.
- No maturity scoring is implemented.
- No static dashboard export is implemented.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#13`.
- Intake recorded: `#24`.
- Story recorded: `US-035`, lane `high-risk`.
- `harness-cli governance report` implemented.
- Default output path implemented:
  `.harness/reports/governance-report.json`.
- Custom `--output` implemented.
- Generated report sections include:
  - repository metadata;
  - story summary;
  - gate summary;
  - validation command summary;
  - release verification summary;
  - friction summary;
  - story-level proof rows.
- Missing evidence is derived into story rows without mutating gates.
- Test added:
  `governance_report_generates_schema_snapshot_without_mutating_gate`.
- CLI help documents `--output`.
- `python scripts/verify-governance-report-schema.py` pass.
- `python scripts/verify-governance-report-schema.py .harness/reports/US-035-governance-report.json`
  pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 47 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `cargo build --package harness-cli --release` pass.
- `target/release/harness-cli.exe governance report --output .harness/reports/US-035-governance-report.json`
  pass.
- Runtime report after story gate:
  - stories: 17;
  - gate pass: 15;
  - gate fail: 1;
  - gate not run: 1;
  - release verification: pass;
  - friction events: 2.
- `harness-cli context --story US-035` pass.
- `harness-cli arch-check --story US-035` pass.
- Detailed trace recorded: `#32`, score `3/3`.
- `harness-cli story verify US-035` pass.
- Story governance gate pass.
- No SQLite migration was added.
- No maturity scoring was implemented.
- No static dashboard export was implemented.
- No release or tag was changed.
