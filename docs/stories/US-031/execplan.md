# Exec Plan

## Goal

Implement read-only backlog suggestions from structured friction events.

## Scope

In scope:

- Add `harness-cli backlog suggest`.
- Add filtering by story, friction type, minimum severity, and limit.
- Group structured friction events into deterministic suggestion rows.
- Prove that backlog rows are not created or closed by suggestion.
- Record intake, context, architecture, validation, and Detailed trace.

Out of scope:

- SQLite migration.
- Durable suggestion table.
- Automatic `backlog add`.
- Rule improvement proposals.
- Policy mutation.
- v0.5 release/tag.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Governance workflow behavior.
- Human-review boundary.
- Public CLI contract.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Add application/domain representation for suggestions.
3. Add repository query and grouping logic.
4. Add CLI command and output table.
5. Add tests proving read-only behavior.
6. Run validation and governance gate.

## Stop Conditions

Pause if:

- The command would write backlog rows by default.
- Suggestions would be treated as accepted policy.
- Free-text trace friction is promoted without structured classification.
