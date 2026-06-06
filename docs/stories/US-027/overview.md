# Overview

## Current Behavior

`harness-cli intake --auto` can read ad hoc CodeGraph JSON or NotebookLM text
from command flags, but it does not prefer validated MCP evidence already
ingested through US-024.

That leaves a gap in v0.4: CodeGraph and NotebookLM artifacts can be validated
and summarized durably, but auto intake still relies on caller-supplied files
instead of the governed evidence trail.

## Target Behavior

When `intake --auto --story <id>` runs, Harness first looks for the latest
passing `context_ingest` evidence for that story.

Passing CodeGraph evidence can seed risk flags, affected docs, and code impact
summary. Passing NotebookLM evidence can seed affected docs and grounded
context. Failed, inconclusive, or missing evidence is never treated as pass.

## Affected Users

- Agents using auto intake after MCP artifacts have been validated.
- Reviewers checking that intake context came from governed evidence.
- Maintainers preparing v0.4 release hardening.

## Non-Goals

- Calling CodeGraph or NotebookLM providers.
- Changing US-023 schemas.
- Writing provider output directly to SQLite.
- Releasing or tagging v0.4.0.
- Changing installer pins.
