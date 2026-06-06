# Design

## Domain Model

`ContextSource` is either `codegraph` or `notebooklm`.

An ingest result contains:

- Story and source identity.
- Source artifact type, id, version, path, and SHA256.
- `pass`, `fail`, or `inconclusive`.
- Mapped context for passing artifacts.
- Diagnostics for failed or inconclusive artifacts.
- Governance eligibility booleans.

The SQLite summary stores identity, paths, hash, result, provenance status,
summary, and failure text. Full provider artifacts remain files.

## Application Flow

```text
context ingest request
  -> read artifact bytes
  -> compute SHA256
  -> parse typed source contract
  -> validate story, source, status, provenance, and claim evidence
  -> map accepted context
  -> write context-ingest-result JSON
  -> write SQLite summary
  -> update latest story intake only on pass
  -> return pass or non-zero fail/inconclusive
```

## Interface Contract

```text
harness-cli context ingest \
  --story US-XXX \
  --source codegraph|notebooklm \
  --file <artifact.json> \
  [--output <result.json>]
```

Story requirements are explicit:

```text
harness-cli story update --id US-XXX --codegraph-ingest 1
harness-cli story update --id US-XXX --notebooklm-ingest 1
```

## Data Model

Migration `006-context-ingest.sql` adds:

- `context_ingest` summary table.
- `story.codegraph_ingest_required`.
- `story.notebooklm_ingest_required`.

## UI / Platform Impact

The command prints source, artifact hash, evidence path, and result. JSON files
and SHA256 behavior are platform-neutral.

## Observability

Every classified ingest attempt writes both an operational result JSON and a
SQLite summary. Context packs list the latest result for each source.

## Alternatives Considered

1. Store full artifact JSON in SQLite. Rejected by Decision 0010.
2. Treat invalid or unavailable artifacts as warnings. Rejected because trust
   failures must not become governance-passing context.
3. Require both sources for every story. Rejected because requirements must be
   explicit and backwards-compatible.
