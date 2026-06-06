# Validation

## Proof Strategy

Prove structured friction capture through local CLI and repository tests. The
tests must show that typed friction is validated, stored durably, queryable,
and rendered in context packs without triggering backlog suggestions or policy
mutation.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Parse valid and invalid friction type, severity, source, and action type. |
| Integration | Capture events into SQLite and query them back. |
| E2E | CLI captures a real friction event for US-030 and context pack renders it. |
| Platform | Migration applies on a local Harness database. |
| Performance | Not applicable. |
| Logs/Audit | Detailed trace records capture semantics and non-goals. |

## Required Failure Cases

- Unknown friction type fails.
- Unknown severity fails.
- Unknown source fails.
- `provider_unavailable` without provider fails.
- `high` severity without evidence fails.
- Invalid `observed_at` fails.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-friction-taxonomy.py
python scripts/verify-mcp-artifact-contracts.py
harness-cli friction add ...
harness-cli query friction-events
harness-cli context --story US-030
harness-cli arch-check --story US-030
harness-cli story verify US-030
```

## Acceptance Criteria

- `friction_event` SQLite table exists.
- `harness-cli friction add` exists.
- `harness-cli query friction-events` exists.
- Valid structured friction event is stored durably.
- Invalid taxonomy values fail.
- Provider-unavailable without provider fails.
- High-severity without evidence fails.
- Story-linked friction appears in context pack.
- Free-text trace friction remains supported.
- No backlog suggestion behavior is added.
- No rule-improvement behavior is added.
- No automatic policy mutation is introduced.
- Detailed trace is `3/3`.
- Story governance gate passes.

## Acceptance Evidence

- GitHub issue created: `#8`.
- Intake recorded: `#19`.
- Story recorded: `US-030`, lane `high-risk`.
- Migration added: `scripts/schema/007-friction-events.sql`.
- Local Harness DB migrated from schema version `6` to `7`.
- `harness-cli friction add --help` documents `friction add`.
- `harness-cli query friction-events` documents and renders structured events.
- Structured friction event captured: `#1`, `release_gap`, severity `high`,
  story `US-030`.
- Context pack generated: `.harness/context/US-030-context.md`.
- Context pack renders `## 7. Structured Friction Events` with the captured
  `release_gap` event.
- `cargo fmt --check` pass.
- `cargo test --workspace` pass: 44 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` pass.
- `python scripts/verify-friction-taxonomy.py` pass.
- `python scripts/verify-mcp-artifact-contracts.py` pass.
- `harness-cli arch-check --story US-030` pass.
- Detailed trace recorded: `#27`, score `3/3`.
- `harness-cli story verify US-030` pass.
- Story governance gate pass.
- `scripts/install-harness.ps1` syntax check pass.
- `scripts/install-harness.sh` syntax check pass.
- Tests cover:
  - valid structured event capture;
  - structured event query;
  - context pack rendering;
  - unknown taxonomy parse failure;
  - provider-unavailable without provider failure;
  - high-severity without evidence failure;
  - invalid `observed_at` failure.
- Free-text `trace.harness_friction` remains supported by existing tests.
- No backlog suggestion engine was added.
- No rule-improvement proposal engine was added.
- No automatic policy mutation was introduced.
