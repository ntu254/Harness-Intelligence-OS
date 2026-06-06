# Exec Plan

## Goal

Define the v0.6 governance report artifact contract before implementing report
generation, maturity scoring, or static dashboard export.

## Scope

In scope:

- Add governance report documentation.
- Add Draft 2020-12 JSON schema.
- Add semantic schema verifier.
- Add Decision 0012.
- Record intake, story, validation, architecture, decision, and trace evidence.

Out of scope:

- CLI report generation.
- Dashboard rendering.
- Maturity scoring.
- SQLite migration.
- Release/tag changes.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Public/internal report contract.
- Governance evidence semantics.
- Future dashboard dependency.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Add governance report docs and schema.
3. Add Decision 0012.
4. Add schema verifier fixtures.
5. Run validation and governance gate.
6. Record Detailed trace.

## Stop Conditions

Pause if:

- The story requires a runtime report generator.
- The schema would hide missing or failed evidence.
- `inconclusive` evidence would be treated as `pass`.
- Dashboard HTML or maturity scoring enters US-034 scope.
