# Validation

## Proof Strategy

Prove rule suggestions are deterministic and read-only. The command can
surface candidate rule improvements, but it must not edit any Harness policy,
decision, schema, or architecture file.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Rule target derivation and filter parsing. |
| Integration | Multiple friction events group into rule proposal rows. |
| E2E | CLI suggests rule proposal from a US-032 friction event. |
| Platform | No migration required; local CLI reads schema 7 database. |
| Performance | Limit bounds output. |
| Logs/Audit | Detailed trace records read-only semantics. |

## Required Failure / Guardrail Cases

- Unknown friction type filter fails.
- Unknown severity filter fails.
- `rules suggest` does not edit Harness docs.
- `rules suggest` does not create decisions.
- `rules suggest` does not create backlog rows.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-friction-taxonomy.py
harness-cli rules suggest --story US-032
harness-cli context --story US-032
harness-cli arch-check --story US-032
harness-cli story verify US-032
```

## Acceptance Criteria

- `harness-cli rules suggest` exists.
- Rule proposals are derived from structured friction events.
- Proposals can filter by story.
- Proposals can filter by friction type.
- Proposals can filter by minimum severity.
- Proposals are deterministic and grouped.
- Rule files are unchanged by suggestion.
- No SQLite migration is added.
- No decision record is created automatically.
- No backlog row is created automatically.
- No automatic policy mutation is introduced.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#10`.
- Intake recorded: `#21`.
- Story recorded: `US-032`, lane `high-risk`.
- `harness-cli rules suggest` implemented as a read-only command.
- Structured friction event captured: `#2`, `ambiguous_policy`, severity
  `medium`, story `US-032`, proposed action `rule_proposal`.
- `harness-cli rules suggest --story US-032` emits one proposal from event
  `#2`.
- `harness-cli rules suggest --story US-032 --type ambiguous-policy --min-severity medium`
  emits the same deterministic proposal.
- Proposal title: `Document read-only rule suggestion boundary`.
- Proposal target: `docs/HARNESS.md`.
- `harness-cli query backlog --open` still shows only existing backlog `#3`;
  no backlog row was created by rule suggestion.
- `harness-cli query decisions` shows no new decision record for the proposal.
- `harness-cli rules suggest --type made-up` fails with unknown friction type.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 46 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `harness-cli context --story US-032` pass.
- `harness-cli arch-check --story US-032` pass.
- Detailed trace recorded: `#29`, score `3/3`.
- `harness-cli story verify US-032` pass.
- Story governance gate pass.
- Tests cover:
  - rule proposal grouping;
  - highest severity preservation;
  - story/type/min-severity filtering;
  - unchanged decisions;
  - unchanged backlog rows.
- No SQLite migration was added.
- No automatic docs/policy edit behavior was added.
- No automatic decision record behavior was added.
- No automatic backlog behavior was added.
- No automatic policy mutation was introduced.
