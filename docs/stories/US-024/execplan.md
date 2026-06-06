# Exec Plan

## Goal

Implement the common validation and ingestion boundary for versioned MCP
artifact files without calling a provider.

## Scope

In scope:

- `context ingest` interface.
- CodeGraph and NotebookLM typed validation.
- Artifact SHA256 and result JSON.
- SQLite summary migration.
- Passing context mapping into the latest linked intake.
- Context pack ingest summaries.
- Explicit story ingest requirements.
- Pass, fail, and inconclusive tests.

Out of scope:

- MCP transport and authentication.
- CodeGraph or NotebookLM artifact production.
- Provider retries.
- v0.4.0 release.

## Risk Classification

Risk flags:

- External systems.
- Public contracts.
- Data model.
- Existing behavior.
- Multi-domain.

Hard gates:

- External provider behavior.
- Durable governance evidence.

## Work Phases

1. Add migration and domain types.
2. Implement typed validators and mapping.
3. Add CLI interface and reporting.
4. Integrate context packs and explicit story gate requirements.
5. Add unit and integration tests.
6. Update docs and installer payload.
7. Run full validation and governance gate.

## Stop Conditions

Pause for human confirmation if:

- Provider calls become necessary.
- Invalid or inconclusive evidence is proposed as passing.
- Full provider payloads must be stored in SQLite.
- Existing stories would become ingest-required by default.
