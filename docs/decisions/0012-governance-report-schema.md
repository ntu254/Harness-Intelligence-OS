# Decision 0012: Governance Report Schema

Date: 2026-06-07

## Status

Accepted

## Context

HI-OS v0.6 needs a stable reporting contract before adding report generation,
maturity summaries, or static dashboard export. Existing Harness evidence is
spread across stories, validations, releases, friction events, and traces. A
dashboard needs a single shape, but report generation must not mutate
governance state or hide missing evidence.

## Decision

Governance reports use static JSON snapshots validated by:

- `docs/schemas/governance-report.schema.json`
- `docs/GOVERNANCE_REPORT.md`
- `scripts/verify-governance-report-schema.py`

The report artifact type is `governance-report`. Runtime report files may be
written under `.harness/reports/` by later stories.

The report contract includes:

- tracked HI-OS product identity;
- repository identity and commit metadata;
- story proof summary;
- governance gate summary;
- validation command summary;
- release verification summary;
- friction and suggestion summary;
- story-level proof rows.

Reports are read-only evidence snapshots. They do not update stories, backlog,
decisions, release proof, friction events, or policy files. Failed or missing
evidence remains visible. `inconclusive` remains distinct from `pass`.

## Alternatives Considered

1. Build the dashboard directly from SQLite queries. Rejected because the
   dashboard would couple to internal tables before v0.6 defines a public
   report boundary.
2. Store generated HTML as the primary evidence. Rejected because JSON is
   easier to validate, diff, archive, and feed into future dashboards.
3. Treat missing evidence as warnings. Rejected because Harness governance must
   preserve fail and inconclusive semantics.

## Consequences

Positive:

- Future report generation can target a stable schema.
- Static dashboard export can consume one validated artifact.
- Release and friction evidence remain visible in a single snapshot.
- US-046 extends the schema with a required `identity` object sourced from
  tracked `hios.toml`.

Tradeoffs:

- US-034 adds only the contract; report generation waits for US-035.
- Maturity scoring and dashboard rendering need separate follow-up stories.

## Follow-Up

- US-035: Implement governance report generation.
- US-036: Add governance maturity summary.
- US-037: Export static dashboard files.
