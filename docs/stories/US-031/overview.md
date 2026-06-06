# Overview

## Current Behavior

US-030 can capture structured friction events, but maintainers still have to
inspect events manually and decide whether they should become backlog work.

## Target Behavior

Harness CLI can suggest backlog candidates from structured friction events
without mutating backlog state.

US-031 adds:

- `harness-cli backlog suggest`
- deterministic suggestion rows grouped from structured friction
- filters for story, type, minimum severity, and limit

## Affected Users

- Maintainers reviewing repeated friction.
- Agents preparing v0.5 follow-up work.
- Future release-hardening stories that need to convert friction into planned
  backlog items.

## Affected Product Docs

- `docs/FRICTION_TAXONOMY.md`
- `docs/HARNESS.md`
- `docs/stories/US-031/validation.md`

## Non-Goals

- Creating backlog rows automatically.
- Closing backlog rows automatically.
- Generating rule-improvement proposals.
- Automatically changing Harness policy.
- Releasing v0.5.0.
