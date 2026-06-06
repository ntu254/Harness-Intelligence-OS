# Design

## Release Evidence Chain

US-028 follows the v0.3 trusted distribution pattern:

```text
release tag
  -> five platform binaries
  -> five SHA256 assets
  -> installer smoke
  -> release verify
  -> SQLite release summary
  -> context pack
  -> story governance gate
  -> Detailed trace
```

## MCP Evidence Chain

The release must document v0.4 adapter evidence without fabricating provider
proof:

```text
CodeGraph adapter
  -> valid codegraph-impact artifact
  -> context ingest pass
  -> auto-intake evidence consumption

NotebookLM adapter
  -> real provider pass if local authenticated provider is available
  -> otherwise provider-unavailable inconclusive artifact
  -> no governance pass claim from inconclusive evidence
```

## Release Gate

The durable US-028 story uses `release_proof_required = 1`, so `story verify
US-028` cannot pass until `harness-cli release verify --version 0.4.0 --story
US-028` stores passing release evidence.

## Guardrails

- Do not modify older release assets.
- Do not change installer pin without matching release assets.
- Do not treat unavailable NotebookLM evidence as pass.
- Do not open v0.5 until v0.4 release evidence is complete or explicitly
  deferred with a trace.
