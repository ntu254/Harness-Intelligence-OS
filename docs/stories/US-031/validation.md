# Validation

## Proof Strategy

Prove that backlog suggestions are deterministic, useful, and read-only. The
command must produce suggestion rows from structured friction events but leave
the `backlog` table unchanged.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Severity ordering and filter parsing. |
| Integration | Multiple friction events group into suggestion rows. |
| E2E | CLI suggests backlog from US-030 friction event #1. |
| Platform | No migration required; local CLI reads schema 7 database. |
| Performance | Limit bounds output. |
| Logs/Audit | Detailed trace records read-only semantics. |

## Required Failure / Guardrail Cases

- Unknown friction type filter fails.
- Unknown severity filter fails.
- `backlog suggest` does not create backlog rows.
- Suggestions do not close backlog rows.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-friction-taxonomy.py
harness-cli backlog suggest --story US-030
harness-cli query backlog --open
harness-cli context --story US-031
harness-cli arch-check --story US-031
harness-cli story verify US-031
```

## Acceptance Criteria

- `harness-cli backlog suggest` exists.
- Suggestions are derived from structured friction events.
- Suggestions can filter by story.
- Suggestions can filter by friction type.
- Suggestions can filter by minimum severity.
- Suggestions are deterministic and grouped.
- Backlog table is unchanged by suggestion.
- No SQLite migration is added.
- No rule-improvement behavior is added.
- No automatic policy mutation is introduced.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#9`.
- Intake recorded: `#20`.
- Story recorded: `US-031`, lane `high-risk`.
- `harness-cli backlog suggest` implemented as a read-only command.
- `harness-cli backlog suggest --story US-030` emits one suggestion from
  structured friction event `#1`.
- `harness-cli backlog suggest --story US-030 --type release-gap --min-severity high`
  emits the same deterministic suggestion.
- Suggestion title: `Review installer payload for v0.5 schema docs`.
- `harness-cli query backlog --open` still shows only existing backlog `#3`;
  no backlog row was created or closed by suggestion.
- `harness-cli backlog suggest --type made-up` fails with unknown friction type.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 45 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `harness-cli context --story US-031` pass.
- `harness-cli arch-check --story US-031` pass.
- Detailed trace recorded: `#28`, score `3/3`.
- `harness-cli story verify US-031` pass.
- Story governance gate pass.
- Tests cover:
  - severity/type parsing;
  - suggestion grouping;
  - highest severity preservation;
  - story/type/min-severity filtering;
  - read-only backlog behavior.
- No SQLite migration was added.
- No automatic `backlog add` behavior was added.
- No rule-improvement behavior was added.
- No automatic policy mutation was introduced.
