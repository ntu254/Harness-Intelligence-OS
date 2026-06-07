# Design

## Domain Model

The packs share one HI-OS contract:

- read operating context before edits;
- classify work through intake;
- use story packets for normal/high-risk work;
- read the story context pack before implementation;
- run validation before claiming done;
- record trace and pass story governance;
- never treat provider `inconclusive` as `pass`.

Each file adapts that contract to the agent surface:

- Cursor: avoid relying on open tabs and visual edits alone.
- Claude Code: account for `CLAUDE.md` loading behavior.
- Codex: align terminal edits, validation, and final handoff.

## Application Flow

```text
agent starts
  -> reads agent pack and Harness entrypoints
  -> runs query matrix
  -> creates or updates story/context
  -> implements scoped change
  -> validates
  -> records trace
  -> runs story verify
  -> hands off with evidence
```

## Interface Contract

No CLI changes. The documentation contract is checked by:

```text
python scripts/verify-adoption-docs.py
```

The verifier checks that all three packs exist, are linked, and include context
pack, no-code-before-validation-path, story verify, provider-inconclusive, and
credential guardrail language.

## Data Model

No SQLite migration is added.

## UI / Platform Impact

The packs include Bash and Windows command forms where useful.

## Observability

The packs require trace and story gate evidence before handoff.

## Alternatives Considered

1. Add one generic agent pack only.
   - Rejected because Cursor, Claude Code, and Codex have different context
     loading surfaces.
2. Change installer behavior in the same story.
   - Rejected because US-042 is adoption documentation; installer propagation
     can be handled separately if needed.

