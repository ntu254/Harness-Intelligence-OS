# Exec Plan

## Goal

Implement a read-only governance report generator that emits schema-valid JSON
for v0.6 dashboards and maturity summaries.

## Scope

In scope:

- Add `harness-cli governance report`.
- Generate report JSON from existing Harness state.
- Add domain/application/infrastructure structs.
- Add tests for schema shape and read-only gate behavior.
- Validate generated reports with the US-034 schema verifier.
- Record intake, context, architecture, validation, and Detailed trace.

Out of scope:

- Maturity scoring.
- Static dashboard export.
- SQLite migration.
- Release/tag changes.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Public CLI contract.
- Governance evidence semantics.
- Dashboard input contract.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Add report domain structs.
3. Add repository generator and JSON writer.
4. Add CLI command.
5. Add tests and generated report schema validation.
6. Run validation and governance gate.

## Stop Conditions

Pause if:

- Report generation would need a migration.
- Report generation would mutate story gates or proof state.
- Maturity scoring or dashboard rendering enters US-035 scope.
