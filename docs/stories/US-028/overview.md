# Overview

## Current Behavior

HI-OS v0.4 has the MCP contract, ingest, CodeGraph adapter, NotebookLM adapter,
and auto-intake evidence integration in place. The public v0.4.0 release has
not yet been cut or verified.

US-026 remains honest about local NotebookLM provider availability: the adapter
exists, but the local `nlm` provider/session is unavailable, so live NotebookLM
proof is inconclusive rather than pass.

## Target Behavior

HI-OS v0.4.0 is released publicly with trusted release evidence and a clear
MCP adapter-layer evidence trail:

- CodeGraph adapter evidence is passing.
- NotebookLM adapter evidence is either passing through a real provider or
  documented as honest inconclusive when the provider is unavailable.
- Inconclusive NotebookLM evidence is not converted into pass.
- Auto intake consumes only passing ingested evidence.
- Release assets and SHA256 files are published and verified.

## Affected Users

- Maintainers cutting public Harness releases.
- Agents relying on v0.4 MCP adapter behavior.
- Reviewers auditing release evidence and governance gates.

## Non-Goals

- Adding new provider capabilities.
- Starting v0.5 learning-loop work.
- Starting v0.6 reporting/dashboard work.
- Weakening `fail` or `inconclusive` semantics.
