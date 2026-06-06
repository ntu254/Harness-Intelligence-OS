# Validation

## Proof Strategy

Prove maturity scoring is deterministic, schema-valid, and read-only.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Maturity score, level, pass percentages, and gap count. |
| Integration | Generated governance report includes `maturity_summary`. |
| E2E | CLI report output validates against the schema. |
| Platform | Windows release binary smoke for maturity output. |
| Performance | Local SQLite reads and integer scoring only. |
| Logs/Audit | Detailed trace records scoring semantics. |

## Required Failure / Guardrail Cases

- Schema rejects score over 100.
- Schema rejects unknown maturity level.
- `inconclusive` release verification does not become pass.
- Missing validation commands produce 0 validation pass percentage.
- Raw report evidence remains present.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-governance-report-schema.py
cargo build --package harness-cli --release
harness-cli governance report --output .harness/reports/US-036-governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/US-036-governance-report.json
harness-cli context --story US-036
harness-cli arch-check --story US-036
harness-cli story verify US-036
```

## Acceptance Criteria

- Governance report schema includes `maturity_summary`.
- Generated reports include maturity score and level.
- Score is bounded from 0 to 100.
- Level is one of `early`, `developing`, `managed`, or `trusted`.
- Gate and validation pass percentages are included.
- Release verification status is included.
- Open governance gap count is included.
- Explanatory notes are included.
- Scoring is read-only.
- No SQLite migration is added.
- No static dashboard export is implemented.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#14`.
- Intake recorded: `#25`.
- Story recorded: `US-036`, lane `high-risk`.
- Governance report schema extended with required `maturity_summary`.
- Schema verifier fixtures updated for maturity summary.
- Generated reports include:
  - score;
  - level;
  - gate pass percent;
  - validation pass percent;
  - release verified flag;
  - open governance gap count;
  - explanatory notes.
- CLI output prints maturity level and score.
- Tests added/updated:
  - `governance_maturity_summary_scores_gaps_deterministically`;
  - generated report snapshot asserts maturity fields.
- Schema rejects:
  - maturity score over 100;
  - unknown maturity level.
- `python scripts/verify-governance-report-schema.py` pass.
- `python scripts/verify-governance-report-schema.py .harness/reports/US-036-governance-report.json`
  pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 48 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `cargo build --package harness-cli --release` pass.
- `target/release/harness-cli.exe governance report --output .harness/reports/US-036-governance-report.json`
  pass.
- Runtime report after story gate:
  - stories: 18;
  - gate pass: 16;
  - gate fail: 1;
  - gate not run: 1;
  - release verification: pass;
  - friction events: 2;
  - maturity: `trusted (90)`.
- `harness-cli context --story US-036` pass.
- `harness-cli arch-check --story US-036` pass.
- Detailed trace recorded: `#33`, score `3/3`.
- `harness-cli story verify US-036` pass.
- Story governance gate pass.
- No SQLite migration was added.
- No static dashboard export was implemented.
- No release or tag was changed.
- No installer pin was changed.
