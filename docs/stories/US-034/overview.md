# Overview

## Current Behavior

Harness can verify stories, releases, MCP artifacts, and structured friction,
but v0.6 does not yet have a stable report artifact that summarizes this
evidence for humans or dashboards.

## Target Behavior

US-034 defines the governance report contract before any report generator or
dashboard renderer is implemented.

US-034 adds:

- `docs/GOVERNANCE_REPORT.md`
- `docs/schemas/governance-report.schema.json`
- `scripts/verify-governance-report-schema.py`
- Decision 0012

## Affected Users

- Maintainers reviewing project governance health.
- Agents preparing v0.6 maturity and dashboard work.
- Future dashboard consumers.

## Affected Product Docs

- `docs/GOVERNANCE_REPORT.md`
- `docs/decisions/0012-governance-report-schema.md`
- `docs/stories/US-034/validation.md`

## Non-Goals

- Implementing `harness-cli governance report`.
- Writing runtime report files.
- Adding maturity scoring.
- Exporting HTML or static dashboard assets.
- Adding a SQLite migration.
- Releasing v0.6.0.
