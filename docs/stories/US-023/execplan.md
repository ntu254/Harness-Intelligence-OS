# Exec Plan

## Goal

Open HI-OS v0.4.0 with stable, versioned MCP artifact contracts before any real
provider adapter or ingestion command is implemented.

## Scope

In scope:

- Define CodeGraph impact JSON Schema.
- Define NotebookLM grounded brief JSON Schema.
- Define context ingest result JSON Schema.
- Define provenance and grounded-claim requirements.
- Define pass/fail/inconclusive and unavailable-source semantics.
- Define mapping into intake, context packs, and story governance.
- Record Decision 0010.
- Add deterministic schema and semantic fixture validation.

Out of scope:

- Live MCP calls.
- Provider authentication or transport.
- CLI command implementation.
- SQLite migration.
- Adapter implementation.
- v0.4.0 release packaging.

## Risk Classification

Risk flags:

- External systems.
- Public contracts.
- Existing behavior.
- Multi-domain.

Hard gates:

- External provider behavior.
- Validation and governance semantics.

## Work Phases

1. Inspect current automated intake and context mapping.
2. Accept the file-based adapter boundary decision.
3. Define three versioned schemas.
4. Define semantic fixtures and contract verification.
5. Generate the US-023 context pack.
6. Run architecture, schema, docs, and existing Rust validation.
7. Record detailed trace and run the story governance gate.

## Stop Conditions

Pause for human confirmation if:

- Providers need direct SQLite access.
- Existing intake behavior must change in this story.
- Inconclusive evidence is proposed as governance-passing.
- A live MCP dependency becomes necessary.
