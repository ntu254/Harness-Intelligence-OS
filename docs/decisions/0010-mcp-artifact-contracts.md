# 0010 MCP Artifact Contracts Use File-Based Adapter Boundaries

Date: 2026-06-07

## Status

Accepted

## Context

HI-OS v0.1 accepts a flat CodeGraph-style JSON report and NotebookLM-style
text during automated intake. Those inputs are useful for proving the context
flow, but they do not define a stable provider boundary, versioned provenance,
or consistent failure semantics.

Real MCP providers can change transport, authentication, response shape, and
availability independently from Harness. Allowing a provider or MCP tool to
write directly into Harness SQLite would couple external behavior to durable
governance state and bypass parse-first validation.

## Decision

MCP integrations use versioned, file-based artifact boundaries.

1. MCP tools and provider adapters do not write directly into Harness SQLite.
2. A provider adapter writes a versioned JSON artifact that conforms to a
   tracked schema under `docs/schemas/`.
3. Harness CLI owns validation, semantic checks, mapping, and durable ingestion.
4. Every grounded claim requires artifact provenance and evidence references or
   citations.
5. `pass` means the artifact is valid and contains the evidence required for
   its declared result.
6. `fail` means the artifact is malformed, violates its contract, or reports a
   deterministic provider/adapter failure.
7. `inconclusive` means the source is unavailable or evidence is insufficient.
   It exits non-zero and never satisfies intake, context pack, or story
   governance.
8. Harness emits a separate `context-ingest-result` artifact so validation and
   governance eligibility remain auditable without trusting provider output.

Initial contracts:

- `docs/schemas/codegraph-impact.schema.json`
- `docs/schemas/notebooklm-brief.schema.json`
- `docs/schemas/context-ingest-result.schema.json`

The anticipated command shape is:

```text
harness-cli context ingest --source codegraph --file impact.json --story US-XXX
harness-cli context ingest --source notebooklm --file brief.json --story US-XXX
```

US-023 defines this contract only. It does not implement those commands.

## Alternatives Considered

1. Let MCP tools write directly to SQLite. Rejected because it bypasses Harness
   validation, couples providers to schema internals, and weakens auditability.
2. Call each MCP provider directly inside intake. Rejected because provider
   transport and availability would become part of the intake contract.
3. Accept unversioned JSON or markdown. Rejected because compatibility,
   provenance, and failure semantics would remain ambiguous.
4. Use versioned files validated and ingested by Harness. Accepted because it
   isolates provider change while preserving deterministic governance.

## Consequences

Positive:

- Provider integrations can evolve without changing the durable Harness model.
- External data crosses one parse-first boundary before governance use.
- Provenance, hashes, claims, and failure states remain inspectable.
- Recorded artifacts can be replayed in tests without live MCP access.

Tradeoffs:

- Adapters must write files and preserve source hashes.
- Harness must implement schema and semantic validation before real adapters
  are useful.
- Artifact schema evolution needs explicit version compatibility rules.

## Follow-Up

- US-024 implements context ingest validation and result generation.
- US-025 implements the CodeGraph impact artifact adapter.
- US-026 implements the NotebookLM grounded brief artifact adapter.
- A future migration may store validated artifact summaries and hashes, but
  provider payloads remain file artifacts rather than direct database writes.
