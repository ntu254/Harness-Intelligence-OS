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
- Structured errors for malformed provider output or adapter validation
  failures.
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

The accepted provider is CodeGraph CLI:

- Executable: `codegraph`.
- Package: `@colbymchenry/codegraph`.
- Authentication: none.
- Project state: `.codegraph/`.
- Initialization: `codegraph init .`.
- Refresh: `codegraph sync .`.
- Default provider invocation: `codegraph affected --stdin --json`.
- Optional provider invocation: `codegraph impact <symbol> --depth <n> --json`.

Harness exposes:

```text
harness-cli codegraph impact \
  --story US-025 \
  --mode changed-files \
  --changed-files <path-list.txt> \
  [--depth 5] \
  [--output <artifact.json>] \
  [--raw-output <provider-response.json>] \
  [--executable codegraph]
```

Symbol mode replaces `--changed-files` with `--symbol <name>`.

The command stores raw provider JSON, normalizes it into the US-023 artifact,
and invokes US-024 ingestion. Missing executables, missing indexes, and
non-zero provider exits are inconclusive. Malformed JSON and invalid provenance
are fail. Both outcomes exit non-zero.

## Design Review

Status: **Accepted / Implemented**.

CodeGraph CLI `0.9.9` was installed and its command help, source JSON shapes,
local index, changed-files analysis, and symbol analysis were exercised.
Windows npm `.cmd` shims are resolved explicitly before subprocess execution.
The provider remains outside SQLite and US-024 remains the only ingestion path.

## Data Model

No schema migration is expected.

Operational provider output and normalized artifacts remain files. US-024
continues to own the existing `context_ingest` SQLite summary.

`.codegraph/` is ignored as local provider state.

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
