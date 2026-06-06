# Overview

## Current Behavior

Automated intake accepts a flat CodeGraph-style JSON report and NotebookLM-style
text. The CLI stores their parsed summaries directly on the intake record.
There is no versioned artifact contract, provenance requirement, or common
pass/fail/inconclusive model for real MCP integrations.

## Target Behavior

HI-OS v0.4 begins with three accepted JSON Schema contracts:

- CodeGraph impact artifact.
- NotebookLM grounded brief artifact.
- Harness context ingest result.

The contracts establish a file boundary between MCP providers and Harness.
Future CLI ingestion validates artifacts before mapping them into intake,
context packs, or story governance.

## Affected Users

- Maintainers implementing MCP adapters.
- Agents relying on automated intake and generated context.
- Reviewers auditing grounded intelligence evidence.

## Affected Product Docs

- `docs/FEATURE_INTAKE.md`
- `docs/HARNESS.md`
- `docs/ARCHITECTURE.md`
- `docs/decisions/0010-mcp-artifact-contracts.md`

## Non-Goals

- Calling CodeGraph, NotebookLM, or any MCP server.
- Implementing `context ingest`.
- Changing SQLite schema or current `intake --auto` behavior.
- Making MCP evidence mandatory for existing stories.
- Implementing US-024, US-025, or US-026.
