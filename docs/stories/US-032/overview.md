# Overview

## Current Behavior

US-031 can suggest backlog candidates from structured friction, but friction
that points at Harness rules, architecture constraints, validation policy, or
source-of-truth gaps still requires manual review from raw events.

## Target Behavior

Harness CLI can suggest rule-improvement proposals from structured friction
events without editing any rule file or policy document.

US-032 adds:

- `harness-cli rules suggest`
- deterministic proposal rows from structured friction
- story/type/min-severity filters

## Affected Users

- Maintainers reviewing Harness policy gaps.
- Agents preparing rule-improvement proposals.
- Future v0.5 release hardening.

## Affected Product Docs

- `docs/FRICTION_TAXONOMY.md`
- `docs/HARNESS.md`
- `docs/stories/US-032/validation.md`

## Non-Goals

- Editing Harness policy automatically.
- Editing architecture rules automatically.
- Creating decision records automatically.
- Creating backlog rows automatically.
- Releasing v0.5.0.
