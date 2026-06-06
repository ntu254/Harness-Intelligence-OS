# Harness Friction Taxonomy

This taxonomy gives Harness traces a stable vocabulary for recurring friction.
It is the contract layer for HI-OS v0.5 learning-loop work.

Friction is evidence that the harness had to compensate for missing context,
ambiguous rules, weak validation, unavailable providers, or repeated manual
steps. A friction event is not a failure by itself and does not mutate policy.
It gives later stories a structured signal they can use to suggest backlog or
rule-improvement work.

## Event Types

| Type | Use when | Typical follow-up |
| --- | --- | --- |
| `missing_context` | Required product, architecture, decision, or story context was absent or stale. | Add or update product docs, context rules, or story packet links. |
| `ambiguous_policy` | Harness rules conflict or do not say what to do. | Propose a policy clarification or decision record. |
| `weak_validation` | Available proof is too shallow, manual, flaky, or missing for the risk lane. | Add tests, smoke checks, or a validation command. |
| `provider_unavailable` | External provider, CLI, auth session, network, notebook, or source is unavailable. | Add preflight checks, setup docs, or alternate provider guidance. |
| `schema_gap` | Existing artifact schema cannot express a legitimate evidence state. | Revise schema in a contract story before implementation. |
| `release_gap` | Release, installer, asset, checksum, or verification evidence is missing or hard to reproduce. | Add release automation or verification coverage. |
| `architecture_rule_gap` | Dependency or boundary rule is missing, stale, or cannot express the current architecture. | Update `harness-architecture.toml` or architecture docs. |
| `repeated_manual_step` | Agents repeatedly perform the same checklist or workaround by hand. | Add a CLI command, template, or documented runbook. |

## Required Event Fields

Structured friction events use `docs/schemas/friction-event.schema.json`.
Capture durable events with `harness-cli friction add`.

Required fields:

- `schema_version`: currently `1.0.0`.
- `artifact_type`: always `friction-event`.
- `event_id`: UUID for the event.
- `friction_type`: one taxonomy type from this document.
- `severity`: `low`, `medium`, or `high`.
- `source`: where the friction was observed.
- `summary`: short human-readable description.
- `observed_at`: RFC 3339 timestamp.

Optional fields add linkage and actionability:

- `story_id`
- `trace_id`
- `provider`
- `affected_paths`
- `evidence`
- `proposed_action`

## Severity

| Severity | Meaning |
| --- | --- |
| `low` | Minor nuisance or one-off manual lookup. |
| `medium` | Repeated friction or a gap that slows normal/high-risk work. |
| `high` | Friction that blocks required evidence, risks a false pass, or affects release/governance reliability. |

## Learning Loop

```text
trace/friction note
  -> structured friction event
  -> `harness-cli backlog suggest`
  -> `harness-cli rules suggest`
  -> human review
  -> accepted harness change
```

## Guardrails

- Friction events do not automatically change policy.
- Provider unavailability remains inconclusive, never pass.
- Runtime artifacts can be ignored by git; durable summaries belong in SQLite
  when captured by `harness-cli friction add`.
- US-029 defines the taxonomy and schema.
- US-030 adds durable structured friction capture.
- US-031 adds read-only backlog suggestions from structured friction.
- US-032 adds read-only rule improvement proposals from structured friction.
