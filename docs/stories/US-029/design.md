# Design

## Domain Model

The friction taxonomy defines recurring Harness friction as typed evidence:

- `missing_context`
- `ambiguous_policy`
- `weak_validation`
- `provider_unavailable`
- `schema_gap`
- `release_gap`
- `architecture_rule_gap`
- `repeated_manual_step`

A friction event has a stable envelope:

- identity: `event_id`, `schema_version`, `artifact_type`
- linkage: `story_id`, `trace_id`
- classification: `friction_type`, `severity`, `source`
- explanation: `summary`, `observed_at`
- optional context: `provider`, `affected_paths`, `evidence`
- optional next step: `proposed_action`

## Application Flow

US-029 is docs and schema only:

```text
trace friction text
  -> taxonomy term
  -> friction-event schema
  -> future durable capture
  -> future suggestion commands
```

US-030 will decide how the CLI captures and stores these events.

## Interface Contract

The public contract for US-029 is the JSON Schema at
`docs/schemas/friction-event.schema.json`.

The schema requires `provider` for `provider_unavailable` events so external
system outages remain attributable. It requires `evidence` for high-severity
events so serious governance friction cannot be recorded without proof.

## Data Model

No SQLite migration is included in US-029. The schema is a file-based contract
for US-030.

## UI / Platform Impact

No UI, dashboard, or platform-shell change.

## Observability

Friction events make trace friction aggregateable without replacing the
human-readable `harness_friction` trace field.

## Alternatives Considered

1. Keep free-text friction only. Rejected because v0.5 needs grouping and
   validation.
2. Add CLI capture and SQLite storage in the same story. Rejected to keep the
   contract stable before implementation.
3. Let friction suggestions mutate policy automatically. Rejected because rule
   changes require human review.
