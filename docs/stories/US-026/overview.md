# Overview

## Current Behavior

US-023 defines the versioned `notebooklm-brief` artifact contract. US-024 can
validate and ingest a supplied NotebookLM artifact, but Harness does not yet
produce that artifact from a real grounded-context provider.

Agents must currently obtain or write a NotebookLM-style file outside the
governed workflow before calling `harness-cli context ingest`.

## Target Behavior

A NotebookLM-compatible adapter obtains grounded context from a documented
provider boundary, captures or references the raw response, normalizes it into
the US-023 `notebooklm-brief` schema, and feeds that artifact through the
existing US-024 ingest path.

Only cited, provenance-backed claims can become grounded context. Provider
unavailability, missing session/auth, insufficient citations, or ambiguous
grounding produce non-passing evidence.

## Affected Users

- Agents requesting product, decision, or architecture context before
  implementation.
- Maintainers configuring NotebookLM-compatible grounded context providers.
- Reviewers auditing citations, source provenance, and governance evidence.

## Affected Product Docs

- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- `docs/decisions/0010-mcp-artifact-contracts.md`
- `docs/schemas/notebooklm-brief.schema.json`

## Non-Goals

- Changing the CodeGraph adapter.
- Writing provider output directly into Harness SQLite.
- Treating uncited summaries as grounded claims.
- Integrating MCP-produced artifacts into auto-intake defaults.
- Releasing or tagging v0.4.0.
- Changing the installer release pin.
- Changing the US-023 schemas unless provider discovery proves a real contract
  defect.
