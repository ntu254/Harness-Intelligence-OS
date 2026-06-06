# Overview

## Current Behavior

US-034 defines the governance report schema, but Harness CLI cannot generate a
runtime report artifact yet.

## Target Behavior

Harness CLI can generate a schema-valid static governance report JSON snapshot.

US-035 adds:

- `harness-cli governance report`
- default output under `.harness/reports/governance-report.json`
- story, gate, validation, release, friction, and story-row summaries
- generated report validation through the US-034 schema verifier

## Affected Users

- Maintainers reviewing governance state.
- Agents preparing v0.6 maturity and dashboard work.
- Future static dashboard export.

## Affected Product Docs

- `docs/GOVERNANCE_REPORT.md`
- `docs/stories/US-035/validation.md`

## Non-Goals

- Maturity scoring.
- Static dashboard export.
- SQLite migration.
- Release/tag changes.
- Mutating story gates during report generation.
