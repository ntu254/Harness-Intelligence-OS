# Design

## Domain Model

An intelligence artifact has:

- A fixed `artifact_type` and semantic `schema_version`.
- A unique artifact or ingest id.
- A target story id.
- `pass`, `fail`, or `inconclusive` status.
- Provider, adapter, invocation, source, revision, and SHA256 provenance.
- Claims tied to evidence references or citations.

`pass` is evidence-bearing. `fail` is a deterministic error. `inconclusive`
means the source or sufficient evidence was unavailable. Only `pass` can be
eligible for governance.

## Application Flow

Future flow:

```text
MCP provider
  -> provider adapter
  -> versioned JSON artifact
  -> Harness schema and semantic validation
  -> context-ingest-result JSON
  -> mapped intake/context fields
  -> optional durable summary
  -> story governance
```

Mapping rules:

| Artifact field | Harness target |
| --- | --- |
| CodeGraph `impact.risk_flags` | Intake risk flags and lane calculation |
| CodeGraph `impact.affected_files` | Impact evidence and context pack |
| CodeGraph `impact.summary` | `code_impact_summary` |
| NotebookLM `brief.affected_docs` | Intake affected docs |
| NotebookLM summary, constraints, claims | `grounded_context` and context pack |
| Ingest status and governance booleans | Future intake/context/story eligibility |
| Artifact id, version, SHA256, claim ids | Future audit summary and trace evidence |

An inconclusive source does not become an empty successful context. It produces
an inconclusive ingest result with all governance eligibility flags false.
Eligibility means the mapped evidence is admissible at that Harness stage. It
does not mean one artifact is sufficient to satisfy every story gate condition.

## Interface Contract

Future command shape:

```text
harness-cli context ingest --source codegraph --file impact.json --story US-XXX
harness-cli context ingest --source notebooklm --file brief.json --story US-XXX
```

Expected command behavior:

- Parse unknown JSON at the CLI boundary.
- Validate the source-specific schema and story id.
- Verify semantic links such as source type and cited claim ids.
- Hash the source artifact.
- Emit a `context-ingest-result` artifact.
- Persist only validated mapped data and an audit summary.
- Exit non-zero for `fail` and `inconclusive`.

No command is implemented in US-023.

## Data Model

No migration is included.

Future storage should retain a summary containing artifact id, type, version,
SHA256, provider, result, report path, story id, and timestamp. Full provider
payloads stay in files and are not copied wholesale into SQLite.

## UI / Platform Impact

None in US-023. JSON files are portable across supported CLI platforms.

## Observability

Artifacts expose invocation ids, source hashes, revisions, claim ids, and
diagnostics. The Harness ingest result records whether an artifact is eligible
for intake, context pack generation, and story verification.

## Alternatives Considered

See Decision `0010-mcp-artifact-contracts`.
