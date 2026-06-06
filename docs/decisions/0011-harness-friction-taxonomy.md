# Decision 0011: Harness Friction Taxonomy

Date: 2026-06-07

## Status

Accepted

## Context

HI-OS already records free-text friction on traces, but free text is hard to
aggregate into learning-loop behavior. v0.5 needs a stable vocabulary before
adding durable friction capture, backlog suggestion, or rule proposal commands.

The taxonomy must preserve Harness governance rules: friction can suggest work,
but it must not automatically mutate policy or convert inconclusive evidence
into pass.

## Decision

Harness friction uses a versioned taxonomy and event schema:

- `docs/FRICTION_TAXONOMY.md` defines the accepted event types.
- `docs/schemas/friction-event.schema.json` defines the file contract.
- US-029 is contract-only and does not add SQLite capture or suggestion logic.
- US-030 may add durable structured friction capture.
- US-031 and US-032 may use captured friction to propose backlog or rule
  improvements, but suggestions require human review before policy changes.

Accepted event types:

- `missing_context`
- `ambiguous_policy`
- `weak_validation`
- `provider_unavailable`
- `schema_gap`
- `release_gap`
- `architecture_rule_gap`
- `repeated_manual_step`

## Alternatives Considered

1. Keep free-text friction only. Rejected because pattern detection would be
   unreliable and hard to validate.
2. Add CLI capture before taxonomy. Rejected because implementation would bake
   in an unstable contract.
3. Let suggestions automatically modify Harness rules. Rejected because policy
   changes need human review and traceability.

## Consequences

Positive:

- Future friction capture can validate a stable schema.
- Backlog suggestions can group repeated friction by type.
- Provider unavailability remains explicit and auditable.

Tradeoffs:

- Existing trace friction remains free text until US-030 implements structured
  capture.
- Some events may need `ambiguous_policy` until a more precise type is accepted
  in a later taxonomy revision.

## Follow-Up

- US-030: Add structured friction capture.
- US-031: Implement backlog suggest from traces.
- US-032: Add rule improvement proposal.
