# Design

## Interface

```text
harness-cli governance report
harness-cli governance report --output .harness/reports/US-035-governance-report.json
```

The default output is:

```text
.harness/reports/governance-report.json
```

## Report Sources

The command reads existing Harness state:

- `story`
- `intake`
- `trace`
- `release_verification`
- `context_ingest`
- `friction_event`
- `backlog`

It also reads git metadata for repository origin, commit, and branch when
available.

## Read-Only Boundary

`governance report` writes only the requested JSON artifact. It does not:

- run story verification;
- run the governance gate;
- update `story.gate_result`;
- create backlog items;
- create rule proposals;
- update release proof;
- update friction events.

Missing evidence is derived into `stories[].missing_evidence` without mutating
the underlying records.

## Output Contract

The output conforms to `docs/schemas/governance-report.schema.json`.
`inconclusive` remains distinct from `pass`; story gate rows use only `pass`,
`fail`, or `not_run` from stored gate state.
