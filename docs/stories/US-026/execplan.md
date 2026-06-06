# Exec Plan

## Goal

Open the NotebookLM grounded-brief adapter story with a provider-first design
review so implementation only starts after a real invocation, session, and
citation contract is accepted.

## Scope

In scope:

- Document the NotebookLM-compatible provider contract.
- Define authentication/session behavior and unavailable states.
- Capture or reference raw provider output.
- Generate `notebooklm-brief` artifacts matching schema version `1.0.0`.
- Preserve source provenance, hashes, retrieval timestamps, and citations.
- Reject uncited or insufficiently grounded claims.
- Feed generated artifacts through `context ingest --source notebooklm`.
- Render passing evidence in the context pack.
- Satisfy the explicit NotebookLM story governance requirement.

Out of scope:

- CodeGraph changes.
- Direct MCP or provider writes to Harness SQLite.
- Auto-intake default integration.
- Unrelated schema or migration changes.
- v0.4.0 release or tag.
- Installer release pin changes.

## Risk Classification

Risk flags:

- External systems.
- Public contracts.
- Audit/security.
- Existing behavior.
- Weak proof until a real provider path is accepted.

Hard gates:

- External provider behavior.
- Authentication or session handling.
- Governance evidence derived from model-generated grounded context.

## Work Phases

1. Confirm the available NotebookLM-compatible provider, invocation protocol,
   authentication/session requirements, and raw response shape.
2. Define the narrow adapter command and response mapping.
3. Verify source provenance and citation semantics.
4. Implement provider invocation and typed normalization without SQLite access.
5. Emit pass, fail, and inconclusive US-023 artifacts.
6. Compose generated artifacts with US-024 ingest and context pack output.
7. Add provenance, citation, unavailable-provider, and platform tests.
8. Run architecture, trace, and story governance verification.

## Stop Conditions

Pause for human confirmation if:

- No real NotebookLM-compatible provider or export path is available.
- Provider credentials or browser session data would need to be committed or
  logged.
- The provider cannot expose citations for each grounded claim.
- Raw output cannot be captured or referenced without leaking sensitive data.
- Implementation requires direct provider-to-SQLite writes.
- The US-023 schema needs a breaking change.
- CodeGraph or auto-intake work becomes necessary to complete the adapter.
