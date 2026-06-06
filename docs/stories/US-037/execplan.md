# Exec Plan

## Goal

Export a standalone static governance dashboard from a schema-valid governance
report JSON artifact.

## Scope

In scope:

- Add `harness-cli governance dashboard`.
- Read report JSON from `--report`.
- Write standalone HTML to `--output`.
- Render maturity, gate, release, friction, and story evidence summaries.
- Add tests for HTML export and escaping.
- Record intake, context, architecture, validation, and Detailed trace.

Out of scope:

- v0.6 release/tag.
- Installer pin changes.
- SQLite migration.
- Live server or dynamic dashboard.
- External assets.

## Risk Classification

Lane: High-Risk.

Risk flags:

- Dashboard evidence rendering.
- Governance report consumption.
- Public CLI contract.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Add dashboard input/result model.
3. Add report JSON loading and static renderer.
4. Add CLI command and tests.
5. Run validation and governance gate.

## Stop Conditions

Pause if:

- Dashboard export would mutate Harness state.
- HTML requires external assets or scripts.
- Release/tag work enters US-037 scope.
