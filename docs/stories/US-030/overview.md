# Overview

## Current Behavior

US-029 defines a friction taxonomy and `friction-event` schema, but Harness can
only record friction as free text on traces. Free text is useful for review but
cannot be validated, grouped, or queried as typed learning-loop evidence.

## Target Behavior

Harness CLI can capture a structured friction event durably in SQLite while
preserving the existing free-text trace friction path.

US-030 adds typed capture only:

- `harness-cli friction add`
- `harness-cli query friction-events`
- context-pack rendering for story-linked friction events

## Affected Users

- Agents recording friction during high-risk tasks.
- Maintainers reviewing repeated Harness pain.
- Future v0.5 stories that suggest backlog or rule improvements from captured
  friction.

## Affected Product Docs

- `docs/FRICTION_TAXONOMY.md`
- `docs/TRACE_SPEC.md`
- `docs/HARNESS.md`
- `docs/stories/US-030/validation.md`

## Non-Goals

- Generating backlog suggestions.
- Generating rule-improvement proposals.
- Automatically changing Harness policy.
- Replacing free-text `trace.harness_friction`.
- Releasing v0.5.0.
