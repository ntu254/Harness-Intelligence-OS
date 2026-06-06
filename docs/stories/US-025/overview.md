# Overview

## Current Behavior

US-023 defines the versioned `codegraph-impact` artifact contract. US-024
validates and ingests a supplied CodeGraph artifact, but Harness does not
produce that artifact from real code intelligence.

Agents must currently create or obtain the input file outside the governed
workflow before calling `harness-cli context ingest`.

## Target Behavior

A CodeGraph-compatible adapter obtains repository impact evidence, normalizes
it into the US-023 `codegraph-impact` schema, writes the artifact to the
file-based boundary, and feeds that file through the existing US-024 ingest
path.

Provider or tool unavailability produces a schema-valid `inconclusive`
artifact. It never becomes passing context or story evidence.

## Affected Users

- Agents requesting code-aware impact evidence before implementation.
- Maintainers configuring or operating CodeGraph-compatible intelligence.
- Reviewers auditing provenance, affected files, risk flags, and governance
  evidence.

## Affected Product Docs

- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- `docs/decisions/0010-mcp-artifact-contracts.md`
- `docs/schemas/codegraph-impact.schema.json`

## Non-Goals

- Implementing the NotebookLM adapter.
- Writing provider output directly into Harness SQLite.
- Changing the US-023 schemas unless implementation proves a real contract
  defect.
- Integrating MCP-produced artifacts into auto-intake defaults.
- Releasing or tagging v0.4.0.
- Changing the installer release pin.
