# Governance Report Contract

HI-OS v0.6 uses governance reports as static JSON snapshots of Harness proof
state. The report is evidence for humans and dashboards; it does not mutate
stories, backlog, policy, or release records.

The canonical schema is:

- `docs/schemas/governance-report.schema.json`

## Report Shape

A governance report captures:

- repository identity and commit metadata;
- story proof summary;
- governance gate summary;
- validation command summary;
- release verification summary;
- friction and suggestion summary;
- story-level proof rows.

## Guardrails

- Reports are read-only snapshots.
- Missing or failed evidence remains visible; it is not downgraded to a warning.
- `inconclusive` remains distinct from `pass`.
- Runtime report files may live under `.harness/reports/`.
- US-034 defines the schema only.
- US-035 implements report generation.
- US-036 adds maturity scoring.
- US-037 exports static dashboard files.
