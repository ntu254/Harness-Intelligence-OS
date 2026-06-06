# Overview

## Current Behavior

US-023 defines versioned CodeGraph, NotebookLM, and context ingest result
schemas. Harness does not yet expose a command that validates provider artifact
files, writes an ingest evidence report, records a durable summary, or maps
accepted context into an intake.

## Target Behavior

`harness-cli context ingest` accepts a tracked artifact file for a story and
source, validates the US-023 contract and trust semantics, hashes the artifact,
writes a `context-ingest-result` JSON report, and stores a SQLite summary.

Only a passing artifact may update intake context. Failed or inconclusive
artifacts remain evidence, exit non-zero, and cannot satisfy explicit story
ingest requirements.

## Affected Users

- Agents ingesting external intelligence before implementation.
- Maintainers implementing future provider adapters.
- Reviewers auditing provenance and story governance.

## Affected Product Docs

- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- `docs/decisions/0010-mcp-artifact-contracts.md`

## Non-Goals

- Calling CodeGraph, NotebookLM, or another MCP server.
- Implementing a provider adapter.
- Releasing or tagging v0.4.0.
- Changing the installer release pin.
- Making ingest evidence mandatory for all existing stories.
