# Design

## Domain Model

A backlog suggestion is a derived view over structured friction events. It is
not durable by itself and does not create or close backlog rows.

Suggested fields:

- `title`
- `friction_type`
- `severity`
- `event_count`
- `stories`
- `pain`
- `suggestion`

## Interface

```text
harness-cli backlog suggest
harness-cli backlog suggest --story US-030
harness-cli backlog suggest --type release_gap --min-severity medium
```

## Suggestion Rules

Suggestions are deterministic:

- Use `proposed_action.title` when the event provides a backlog-oriented title.
- Otherwise derive a title from the friction type and summary.
- Group identical title and friction type pairs.
- Preserve the highest severity in the group.
- Include linked stories as comma-separated story ids.

Severity order:

```text
low < medium < high
```

## Guardrails

- `backlog suggest` is read-only.
- Suggestions never mark story proof as pass.
- Suggestions never mutate Harness policy.
- Suggestions do not replace human review.

## Data Model

No SQLite migration is added in US-031. The command reads `friction_event`
records added by US-030.
