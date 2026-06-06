# Design

## Domain Model

The adapter translates CodeGraph-compatible analysis into the existing
`codegraph-impact` artifact:

- Artifact identity and schema version.
- Story identity.
- `pass`, `fail`, or `inconclusive` status.
- Provider, adapter, invocation, repository, revision, and hashed-input
  provenance.
- Impact summary, affected files, dependency edges, risk flags, and grounded
  claims for passing analysis.
- Structured errors for deterministic failures.
- Structured unavailability for inconclusive analysis.

Harness does not treat raw provider output as trusted context.

## Application Flow

```text
CodeGraph-compatible analysis
  -> adapter parses provider response
  -> adapter maps US-023 artifact
  -> write codegraph-impact.json
  -> context ingest --source codegraph
  -> US-024 schema and semantic validation
  -> SQLite ingest summary
  -> intake and context pack mapping on pass
  -> explicit story governance evidence
```

The adapter owns provider translation. The existing context ingest path remains
the only component allowed to map provider evidence into Harness state.

## Interface Contract

The final CLI shape must:

- Accept a story id and output path.
- Identify the repository and revision being analyzed.
- Produce a `codegraph-impact` file even for fail or inconclusive outcomes.
- Optionally invoke US-024 ingest as the next explicit step or as a documented
  composed workflow.
- Exit non-zero for fail and inconclusive outcomes.

The exact provider transport, authentication, and command flags are resolved
during implementation discovery from the real CodeGraph-compatible tool. Do
not invent a provider protocol or couple it to SQLite.

## Design Review

Status: **Conditionally accepted**.

The artifact flow, ownership boundary, status semantics, and governance
composition are accepted. Implementation may begin only after one real
CodeGraph-compatible invocation boundary is identified with:

- An executable, MCP tool, HTTP endpoint, or other callable interface.
- Authentication and secret-handling requirements.
- A response shape that can ground affected files, dependency edges, and
  claims.
- Provider unavailability and deterministic failure signals.
- A repeatable test or fixture strategy.

Workspace review on 2026-06-07 found no CodeGraph executable, environment
configuration, MCP resource, connector, or deferred tool. US-025 therefore
remains `planned`; a synthetic provider interface is not accepted as real
CodeGraph evidence.

## Data Model

No schema migration is expected.

Operational provider output and normalized artifacts remain files. US-024
continues to own the existing `context_ingest` SQLite summary.

## UI / Platform Impact

The adapter is a CLI workflow. Paths, hashes, repository revisions, exit codes,
and generated JSON must behave consistently across supported platforms.

## Observability

The generated artifact records invocation and source provenance. The composed
ingest records artifact SHA256, result, diagnostics, and report path. Detailed
trace evidence records provider availability and adapter behavior without
storing credentials.

## Alternatives Considered

1. Let the provider write directly to SQLite. Rejected by Decision 0010.
2. Pass raw provider output directly to intake. Rejected because it bypasses
   the stable artifact and validation boundary.
3. Implement CodeGraph and NotebookLM together. Rejected to keep provider
   failures and validation evidence attributable to one adapter.
4. Hide unavailable analysis as an empty successful report. Rejected because
   unavailable evidence is inconclusive, never pass.
