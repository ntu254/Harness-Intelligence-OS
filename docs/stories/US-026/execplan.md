# Exec Plan

## Goal

Open the NotebookLM grounded-brief adapter story with a provider-first design
review. The default provider boundary is accepted, and implementation starts
only after a real local invocation/session path can be exercised without
storing secrets in Harness.

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

1. Accepted: default provider is `notebooklm-mcp-cli`, invoked through local
   CLI executable `nlm`, with optional MCP server executable `notebooklm-mcp`.
2. Implemented: `harness-cli notebooklm brief` invokes `nlm ask --json
   --query <query>` with optional `--notebook`.
3. Implemented: raw provider stdout/stderr is written to a raw response file
   when available.
4. Implemented: typed normalization maps cited provider output into the US-023
   `notebooklm-brief` schema without SQLite access.
5. Implemented: pass, fail, and inconclusive artifacts compose with US-024
   ingest.
6. Implemented: tests cover valid cited output, invalid provider JSON,
   uncited claims, and missing provider executable.
7. Pending live proof: confirm a local authenticated provider session and
   capture or reference a safe raw grounded response.
8. Pending live proof: run context pack rendering and final story governance
   verification with valid NotebookLM evidence.
9. Pending hardening: run the full workspace validation ladder and record final
   trace.

## Stop Conditions

Pause for human confirmation if:

- No real NotebookLM-compatible provider or export path is available.
- Provider credentials or browser session data would need to be committed or
  logged.
- Harness would need to store Google credentials, cookies, browser profiles,
  tokens, or provider session files.
- The provider is missing, unauthenticated, expired, network-blocked, or
  points at a missing notebook; record `inconclusive` instead of continuing as
  a passing proof.
- The provider cannot expose citations for each grounded claim.
- Raw output cannot be captured or referenced without leaking sensitive data.
- Implementation requires direct provider-to-SQLite writes.
- The US-023 schema needs a breaking change.
- CodeGraph or auto-intake work becomes necessary to complete the adapter.
