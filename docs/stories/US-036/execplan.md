# Exec Plan

## Goal

Add deterministic maturity scoring to governance reports without introducing a
dashboard, migration, release, or policy mutation.

## Scope

In scope:

- Extend governance report schema with `maturity_summary`.
- Update schema verifier fixtures.
- Add maturity summary domain model.
- Compute maturity summary during report generation.
- Print maturity level and score in CLI smoke output.
- Add tests for scoring and generated report shape.
- Record intake, context, architecture, validation, and Detailed trace.

Out of scope:

- Static dashboard export.
- SQLite migration.
- Release/tag changes.
- Installer pin changes.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Governance scoring semantics.
- Dashboard input contract.
- Release-readiness interpretation.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Extend report schema and verifier fixtures.
3. Add scoring model and generator integration.
4. Add tests and CLI smoke.
5. Run validation and governance gate.

## Stop Conditions

Pause if:

- Scoring would hide raw evidence.
- `inconclusive` would become equivalent to pass.
- Dashboard rendering enters US-036 scope.
