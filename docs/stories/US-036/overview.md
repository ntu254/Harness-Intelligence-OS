# Overview

## Current Behavior

US-035 can generate governance report JSON, but the report has no maturity
summary for maintainers or future dashboards.

## Target Behavior

Generated governance reports include a deterministic maturity summary derived
from existing Harness evidence.

US-036 adds:

- `maturity_summary` to the governance report schema
- integer score and level
- gate and validation pass percentages
- release verification status
- open governance gap count
- explanatory notes

## Affected Users

- Maintainers reviewing HI-OS governance readiness.
- Agents preparing v0.6 release hardening.
- Future static dashboard consumers.

## Affected Product Docs

- `docs/GOVERNANCE_REPORT.md`
- `docs/schemas/governance-report.schema.json`
- `docs/stories/US-036/validation.md`

## Non-Goals

- Static dashboard export.
- SQLite migration.
- Release/tag changes.
- Installer pin changes.
- Changing story gates or proof state during scoring.
