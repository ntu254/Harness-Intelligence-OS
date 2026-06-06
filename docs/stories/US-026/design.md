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

Status: **Provider contract accepted for implementation planning**.

Default provider:

- Provider package: `notebooklm-mcp-cli`.
- Provider boundary: local CLI first, optional MCP server second.
- Primary executable: `nlm`.
- Optional MCP server executable: `notebooklm-mcp`.
- Authentication: interactive Google login/session managed by the provider.
- Network: required for provider operations.
- Provider state: external to Harness.

Harness must not store Google credentials, cookies, browser profiles, tokens,
or provider session files. Missing executable, missing authentication, expired
session, missing notebook, network failure, provider timeout, permission denial,
or insufficient provider evidence produces `inconclusive`, never `pass`.

The adapter captures or references raw provider output, then normalizes it into
the US-023 `notebooklm-brief` schema. The normalized artifact must keep the
existing schema shape:

- `schema_version`: `1.0.0`.
- `artifact_type`: `notebooklm-brief`.
- `provenance.provider`: `notebooklm-mcp-cli`.
- `provenance.adapter`: `harness-cli-notebooklm`.
- `provenance.adapter_version` and `provenance.invocation_id`.
- `provenance.sources[]` entries with `source_id`, `title`, `uri`, `sha256`,
  and `retrieved_at`.
- `brief.claims[].citations[]` entries that reference a known `SRC-*` source
  and locator.

NotebookLM prose is not grounded evidence unless every material claim maps to
a citation. Raw response that cannot be parsed, schema-invalid artifacts,
missing provenance, missing citations, unknown cited sources, or hash mismatch
produce `fail`.

Alternate provider:

- `PleasePrompto/notebooklm-mcp` remains an MCP-oriented fallback candidate
  because it exposes NotebookLM through an MCP server with browser/session
  management and DOM-level citations.
- It is not the default for US-026 unless the default provider proves unusable.

The anticipated Harness command shape is:

```text
harness-cli notebooklm brief \
  --story US-026 \
  --source-pack <provider-specific-source-or-export> \
  [--output <artifact.json>] \
  [--raw-output <provider-response.json>]
```

Exact flags are not accepted yet. They must come from the real provider
implementation pass and the observed provider response, while preserving the
accepted trust boundary above.

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
