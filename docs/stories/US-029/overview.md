# Overview

## Current Behavior

Harness traces can store free-text friction. That is useful for humans but too
loose for v0.5 learning-loop commands that need to group repeated failures,
provider outages, release gaps, or missing validation patterns.

## Target Behavior

HI-OS has a contract-first friction taxonomy and versioned event schema. Future
stories can capture structured friction durably and suggest backlog or rule
improvement work from those events.

US-029 defines the vocabulary and schema only. It does not add CLI commands,
SQLite tables, or automated policy mutation.

## Affected Users

- Agents recording traces and friction.
- Maintainers reviewing repeated harness pain.
- Future v0.5 stories that suggest backlog and rule improvements.

## Affected Product Docs

- `docs/FRICTION_TAXONOMY.md`
- `docs/schemas/friction-event.schema.json`
- `docs/decisions/0011-harness-friction-taxonomy.md`
- `docs/TRACE_SPEC.md`

## Non-Goals

- Adding `harness-cli trace friction add`.
- Adding SQLite friction-event tables.
- Generating backlog suggestions.
- Generating rule-improvement proposals.
- Automatically changing Harness policy.
- Releasing v0.5.0.
