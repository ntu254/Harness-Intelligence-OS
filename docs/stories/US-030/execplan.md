# Exec Plan

## Goal

Implement structured friction capture so v0.5 learning-loop work can rely on
typed, validated friction events instead of free-text traces only.

## Scope

In scope:

- Add SQLite migration `007-friction-events.sql`.
- Add domain types for friction type, severity, source, and action type.
- Add `harness-cli friction add`.
- Add `harness-cli query friction-events`.
- Render story-linked friction events in context packs.
- Add tests for pass/fail semantics.
- Record intake, context, architecture, validation, and Detailed trace.

Out of scope:

- Backlog suggestion generation.
- Rule improvement proposal generation.
- Automatic policy mutation.
- v0.5 release/tag.

## Risk Classification

Lane: High-Risk.

Risk flags:

- SQLite migration.
- Public CLI contract.
- Governance learning-loop evidence.
- Audit/security evidence semantics.

## Work Phases

1. Open GitHub issue and durable Harness story.
2. Add schema migration.
3. Add application/domain/interface/storage support.
4. Add context-pack rendering.
5. Add unit/integration tests.
6. Capture a real structured friction event.
7. Run governance validation and trace.

## Stop Conditions

Pause if implementation would:

- Promote inconclusive provider evidence to pass.
- Auto-create or close backlog items.
- Change story proof flags from friction capture alone.
- Require credentials or external provider calls.
