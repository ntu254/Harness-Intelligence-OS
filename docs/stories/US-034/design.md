# Design

## Contract

The governance report is a static JSON artifact:

- schema version: `1.0.0`
- artifact type: `governance-report`
- schema: `docs/schemas/governance-report.schema.json`

The report captures a point-in-time governance snapshot. It is not an
operational command result and it does not mutate Harness state.

## Report Sections

The schema defines:

- `repository`
- `story_summary`
- `gate_summary`
- `validation_summary`
- `release_summary`
- `friction_summary`
- `stories`

Story rows include proof flags and `gate_result`. Release verification can be
`pass`, `fail`, `inconclusive`, or `none`. Story gate results are only `pass`,
`fail`, or `not_run`; inconclusive evidence must remain visible through the
relevant evidence fields rather than being treated as a gate pass.

## Guardrails

- Reports are read-only snapshots.
- Reports do not create or update stories.
- Reports do not create backlog suggestions or rule proposals.
- Reports do not update decision records.
- Reports do not update release proof.
- Missing evidence is preserved.
- `inconclusive` is never converted into `pass`.

## Runtime Boundary

US-034 only defines the file contract. Later stories may write runtime report
artifacts under:

```text
.harness/reports/
```

## Verification

`scripts/verify-governance-report-schema.py` validates the schema and semantic
fixtures, including invalid examples for warning-like states, negative counts,
extra properties, and invalid story gate semantics.
