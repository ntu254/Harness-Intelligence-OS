# Design

## Domain Model

The adapter translates NotebookLM-compatible grounded context into the existing
`notebooklm-brief` artifact:

- Artifact identity and schema version.
- Story identity.
- `pass`, `fail`, or `inconclusive` status.
- Provider, adapter, invocation, and source provenance.
- Source ids, source titles, URIs, source hashes, and retrieval timestamps.
- Brief summary, constraints, open questions, affected docs, and grounded
  claims.
- Citations for every grounded claim.
- Structured errors for malformed or ungrounded provider output.
- Structured unavailability for provider/session/source failures.

Harness does not treat raw provider output or model-generated prose as trusted
grounded context.

## Application Flow

```text
NotebookLM-compatible provider
  -> adapter captures raw grounded response
  -> adapter verifies source provenance and citations
  -> adapter maps US-023 notebooklm-brief artifact
  -> write notebooklm-brief.json
  -> context ingest --source notebooklm
  -> US-024 schema and semantic validation
  -> SQLite ingest summary
  -> intake grounded_context and context pack mapping on pass
  -> explicit story governance evidence
```

The adapter owns provider translation. US-024 remains the only component
allowed to map provider evidence into Harness state.

## Interface Contract

Status: **Provider contract pending**.

Implementation cannot begin until one NotebookLM-compatible boundary is
accepted with:

- Provider identity and versioning.
- Invocation mechanism: executable, MCP tool, browser automation, API, or
  exported file.
- Authentication/session behavior and secret-handling rules.
- Raw response shape or export format.
- Source provenance and hash strategy.
- Citation format that can map each claim to a `SRC-*` source and locator.
- Provider unavailable, permission denied, timeout, source unavailable, and
  insufficient-evidence signals.
- Deterministic fixtures for pass, fail, and inconclusive behavior.

The anticipated Harness command shape is:

```text
harness-cli notebooklm brief \
  --story US-026 \
  --source-pack <provider-specific-source-or-export> \
  [--output <artifact.json>] \
  [--raw-output <provider-response.json>]
```

Exact flags are not accepted yet. They must come from the real provider
contract, not from an invented protocol.

## Data Model

No schema migration is expected.

Operational provider output and normalized artifacts remain files. US-024
continues to own the existing `context_ingest` SQLite summary.

## UI / Platform Impact

The adapter is expected to be a CLI workflow. If the provider requires browser
session state or interactive authentication, implementation must document the
setup and produce inconclusive evidence when the session is unavailable.

## Observability

The generated artifact records invocation and source provenance. The composed
ingest records artifact SHA256, result, diagnostics, and report path. Detailed
trace evidence records provider/session availability without storing secrets.

## Alternatives Considered

1. Treat NotebookLM summary text as grounded context. Rejected because claims
   without citations are not auditable.
2. Let NotebookLM or an MCP tool write directly to SQLite. Rejected by
   Decision 0010.
3. Implement NotebookLM together with CodeGraph. Rejected because US-025 is
   complete and provider risks should remain attributable.
4. Mark provider unavailable as pass with empty context. Rejected because
   unavailability is inconclusive.
