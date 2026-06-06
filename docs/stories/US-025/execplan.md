# Exec Plan

## Goal

Implement one CodeGraph-compatible producer adapter that creates a valid
US-023 impact artifact and composes with US-024 ingestion without weakening the
file-based trust boundary.

## Scope

In scope:

- Discover and document the real CodeGraph-compatible invocation boundary.
- Generate `codegraph-impact` artifacts matching schema version `1.0.0`.
- Preserve repository, revision, invocation, input hash, and adapter
  provenance.
- Map affected files, dependency edges, risk flags, summaries, and claims.
- Emit schema-valid fail or inconclusive artifacts when analysis cannot pass.
- Feed generated artifacts through `context ingest --source codegraph`.
- Render passing evidence in the context pack.
- Satisfy the explicit CodeGraph story governance requirement.

Out of scope:

- NotebookLM integration.
- Direct MCP or provider writes to Harness SQLite.
- Auto-intake default integration.
- Unrelated schema or migration changes.
- v0.4.0 release or tag.
- Installer release pin changes.

## Risk Classification

Risk flags:

- External systems.
- Public contracts.
- Existing behavior.
- Cross-platform.
- Weak proof until a real provider path is exercised.

Hard gates:

- External provider behavior.
- Governance evidence derived from external analysis.

## Work Phases

1. Confirm the available CodeGraph-compatible provider, invocation protocol,
   authentication requirements, and deterministic fixture strategy.
2. Define the narrow adapter command and response mapping.
3. Implement provider invocation and typed normalization without SQLite access.
4. Emit pass, fail, and inconclusive US-023 artifacts.
5. Compose the generated artifact with US-024 ingest and context pack output.
6. Add unit, integration, unavailable-provider, and platform tests.
7. Run architecture, trace, and story governance verification.

## Stop Conditions

Pause for human confirmation if:

- No real CodeGraph-compatible tool or callable provider is available.
- Provider credentials would need to be committed or logged.
- The provider cannot expose enough provenance for grounded claims.
- Implementation requires direct provider-to-SQLite writes.
- The US-023 schema needs a breaking change.
- NotebookLM or auto-intake work becomes necessary to complete the adapter.
