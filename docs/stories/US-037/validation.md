# Validation

## Proof Strategy

Prove dashboard export reads a governance report JSON artifact and writes a
standalone static HTML file without mutating Harness state.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | HTML renderer escapes story content. |
| Integration | Dashboard command reads report JSON and writes HTML. |
| E2E | CLI generates report, validates schema, exports dashboard. |
| Platform | Windows release binary smoke for dashboard export. |
| Performance | Static rendering is local file IO only. |
| Logs/Audit | Detailed trace records dashboard read-only boundary. |

## Required Failure / Guardrail Cases

- Missing report file fails.
- Non-governance report fails.
- Export does not run story verification.
- Export does not mutate SQLite.
- Export uses no external assets or scripts.
- HTML escapes story content.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-governance-report-schema.py
cargo build --package harness-cli --release
harness-cli governance report --output .harness/reports/US-037-governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/US-037-governance-report.json
harness-cli governance dashboard --report .harness/reports/US-037-governance-report.json --output .harness/dashboard/US-037-index.html
harness-cli context --story US-037
harness-cli arch-check --story US-037
harness-cli story verify US-037
```

## Acceptance Criteria

- `harness-cli governance dashboard` exists.
- `--report` is supported.
- `--output` is supported.
- Default report path is `.harness/reports/governance-report.json`.
- Default dashboard path is `.harness/dashboard/index.html`.
- Dashboard renders maturity, gate, release, friction, and story evidence.
- Dashboard is standalone static HTML.
- Dashboard uses no external assets or scripts.
- Export is read-only except for the output HTML file.
- No SQLite migration is added.
- No release or tag is changed.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#15`.
- Intake recorded: `#26`.
- Story recorded: `US-037`, lane `high-risk`.
- `harness-cli governance dashboard` implemented.
- `--report` and `--output` supported.
- Default report path implemented:
  `.harness/reports/governance-report.json`.
- Default dashboard path implemented:
  `.harness/dashboard/index.html`.
- Dashboard renders:
  - maturity level and score;
  - story count;
  - gate pass/fail/not-run counts;
  - release verification result;
  - friction event count;
  - open governance gap count;
  - maturity notes;
  - story evidence rows.
- Dashboard output is standalone static HTML.
- Dashboard uses no external assets or scripts.
- HTML escaping covered by test fixture.
- Test added:
  `governance_dashboard_exports_static_html_from_report`.
- `python scripts/verify-governance-report-schema.py` pass.
- `python scripts/verify-governance-report-schema.py .harness/reports/US-037-governance-report.json`
  pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 49 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `cargo build --package harness-cli --release` pass.
- `target/release/harness-cli.exe governance report --output .harness/reports/US-037-governance-report.json`
  pass.
- `target/release/harness-cli.exe governance dashboard --report .harness/reports/US-037-governance-report.json --output .harness/dashboard/US-037-index.html`
  pass.
- Runtime dashboard after story gate:
  - stories: 19;
  - gate pass: 17;
  - maturity: `trusted (90)`.
- HTML smoke confirms `HI-OS Governance Dashboard` and maturity content.
- `harness-cli context --story US-037` pass.
- `harness-cli arch-check --story US-037` pass.
- Detailed trace recorded: `#34`, score `3/3`.
- `harness-cli story verify US-037` pass.
- Story governance gate pass.
- No SQLite migration was added.
- No release or tag was changed.
- No installer pin was changed.
