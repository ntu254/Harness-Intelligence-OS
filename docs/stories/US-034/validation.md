# Validation

## Proof Strategy

Prove the governance report contract is schema-valid, semantically guarded, and
recorded in Harness as a high-risk planning story without implementing report
generation.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | JSON schema validity and invalid fixture rejection. |
| Integration | Contract docs, Decision 0012, and verifier alignment. |
| E2E | Harness context, architecture check, decision verify, story gate. |
| Platform | No platform packaging change. |
| Performance | Not applicable. |
| Logs/Audit | Detailed trace records contract-only boundary. |

## Required Failure / Guardrail Cases

- Invalid artifact type fails.
- Warning-like release result fails.
- Warning-like validation result fails.
- Story gate `inconclusive` fails.
- Negative counts fail.
- Additional root properties fail.
- Missing gate result fails.

## Commands

```text
python scripts/verify-governance-report-schema.py
python scripts/verify-friction-taxonomy.py
python scripts/verify-mcp-artifact-contracts.py
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
harness-cli context --story US-034
harness-cli arch-check --story US-034
harness-cli decision verify 0012-governance-report-schema
harness-cli story verify US-034
```

## Acceptance Criteria

- Governance report schema exists.
- Governance report docs exist.
- Decision 0012 is accepted and verified.
- Schema uses Draft 2020-12.
- Valid governance report fixture passes.
- Invalid semantic fixtures fail.
- `inconclusive` remains distinct from `pass`.
- Story summary, gate summary, validation summary, release summary, friction
  summary, and story rows are represented.
- No CLI report generation is implemented.
- No dashboard export is implemented.
- No SQLite migration is added.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#12`.
- Intake recorded: `#23`.
- Story recorded: `US-034`, lane `high-risk`.
- Decision recorded: `0012-governance-report-schema`, status `accepted`.
- Governance report docs added: `docs/GOVERNANCE_REPORT.md`.
- Governance report schema added:
  `docs/schemas/governance-report.schema.json`.
- Governance report verifier added:
  `scripts/verify-governance-report-schema.py`.
- Decision 0012 added:
  `docs/decisions/0012-governance-report-schema.md`.
- Story packet added under `docs/stories/US-034/`.
- `python scripts/verify-governance-report-schema.py` pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 46 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `harness-cli context --story US-034` pass.
- `harness-cli arch-check --story US-034` pass.
- `harness-cli decision verify 0012-governance-report-schema` pass.
- Detailed trace recorded: `#31`, score `3/3`.
- `harness-cli story verify US-034` pass.
- Story governance gate pass.
- Verifier rejects:
  - invalid artifact type;
  - warning-like release result;
  - warning-like validation result;
  - story gate `inconclusive`;
  - negative counts;
  - additional root properties;
  - missing gate result.
- No CLI report generation was implemented.
- No dashboard export was implemented.
- No maturity scoring was implemented.
- No SQLite migration was added.
- No release or tag was changed.
