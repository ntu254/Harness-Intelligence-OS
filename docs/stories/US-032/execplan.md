# Exec Plan

## Goal

Implement read-only rule-improvement proposals from structured friction events.

## Scope

In scope:

- Add `harness-cli rules suggest`.
- Add filtering by story, friction type, minimum severity, and limit.
- Derive deterministic proposal rows from structured friction.
- Prove rule files are not edited by suggestion.
- Record intake, context, architecture, validation, and Detailed trace.

Out of scope:

- SQLite migration.
- Durable proposal table.
- Automatic docs or policy edits.
- Automatic decision records.
- v0.5 release/tag.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Governance policy boundary.
- Public CLI contract.
- Human-review rule changes.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Add application/domain representation for rule proposals.
3. Add repository query and grouping logic.
4. Add CLI command and output table.
5. Add tests proving read-only behavior.
6. Run validation and governance gate.

## Stop Conditions

Pause if:

- The command would edit docs, decisions, schemas, or architecture rules.
- Suggestions would be treated as accepted policy.
- Rule proposals bypass human review.
