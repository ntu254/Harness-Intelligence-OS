# Design

## Domain Model

A rule proposal is a read-only derived view over structured friction events.
It is not a durable policy change and does not edit files.

Suggested fields:

- `title`
- `friction_type`
- `severity`
- `target`
- `stories`
- `rationale`
- `proposal`

## Interface

```text
harness-cli rules suggest
harness-cli rules suggest --story US-032
harness-cli rules suggest --type ambiguous-policy --min-severity medium
```

## Proposal Rules

Suggestions are deterministic:

- Use `proposed_action.title` and `target_path` when action type is
  `rule_proposal`.
- Otherwise derive target and title from `friction_type`.
- Group identical title and target pairs.
- Preserve the highest severity in the group.
- Include linked stories.

## Default Targets

| Friction type | Default target |
| --- | --- |
| `ambiguous_policy` | `docs/HARNESS.md` |
| `architecture_rule_gap` | `harness-architecture.toml` |
| `schema_gap` | `docs/schemas/` |
| `weak_validation` | `docs/TEST_MATRIX.md` |
| `missing_context` | `docs/CONTEXT_RULES.md` |
| `provider_unavailable` | `docs/HARNESS.md` |
| `release_gap` | `docs/stories/US-033/overview.md` |
| `repeated_manual_step` | `docs/HARNESS.md` |

## Guardrails

- `rules suggest` is read-only.
- Suggestions do not modify docs, decisions, schemas, or architecture rules.
- Suggestions do not mark story proof as pass.
- Suggestions require human review before any rule changes.

## Data Model

No SQLite migration is added in US-032. The command reads `friction_event`
records added by US-030.
