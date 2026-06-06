# Exec Plan

## Goal

Define the v0.5 friction taxonomy contract before adding durable friction
capture or suggestion commands.

## Scope

In scope:

- Add `docs/FRICTION_TAXONOMY.md`.
- Add `docs/schemas/friction-event.schema.json`.
- Add Decision 0011.
- Add deterministic schema verifier and fixtures.
- Record intake, context, architecture, validation, and trace evidence.

Out of scope:

- SQLite migration.
- `trace friction add`.
- `backlog suggest`.
- `rules suggest`.
- Dashboard/reporting work.
- v0.5 release/tag.

## Risk Classification

Risk flags:

- Public contracts.
- Audit/security.
- Existing behavior.
- Weak proof until schema verifier exists.

Hard gates:

- Harness policy/validation contract.
- Future governance learning loop.

## Work Phases

1. Open v0.5 milestone and US-029 story.
2. Define taxonomy and event schema.
3. Record Decision 0011.
4. Add verifier fixtures for valid and invalid events.
5. Run validation.
6. Record Detailed trace and governance gate.

## Stop Conditions

Pause for human confirmation if:

- Taxonomy requires automatic policy mutation.
- The schema needs durable storage fields that imply a migration.
- Existing trace semantics would be weakened.
- Provider unavailable could be interpreted as pass.
