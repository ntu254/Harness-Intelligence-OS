# Design

## Evidence Source

US-027 uses the existing `context_ingest` SQLite summary as the durable index
and reads the corresponding JSON ingest report for `mapped_context`.

Only rows with `result = 'pass'` are eligible. `fail`, `inconclusive`, and
missing rows remain visible in context packs and story gates but do not seed
auto intake.

## Auto Intake Flow

```text
intake --auto --story US-XXX
  -> read latest passing codegraph context_ingest report
  -> read latest passing notebooklm context_ingest report
  -> merge mapped_context into intake defaults
  -> fall back to explicit --impact-report / --business-context only when no
     passing ingested evidence exists for that source
  -> record intake
```

## Precedence

Validated evidence is preferred over ad hoc command input because it has already
passed schema, provenance, and semantic checks.

Fallback command inputs stay supported for pre-US-024 workflows or stories that
have not produced validated evidence yet.

## Failure Semantics

- Passing CodeGraph evidence seeds risk flags, affected docs, and code impact.
- Passing NotebookLM evidence seeds affected docs and grounded context.
- Failed evidence does not seed intake.
- Inconclusive evidence does not seed intake.
- Missing required proof remains enforced by story governance, not by intake
  creation, to avoid a circular bootstrap between intake and context ingest.

## Boundaries

The implementation is read-only with respect to MCP artifacts. It does not call
providers and does not introduce direct provider-to-SQLite writes.
