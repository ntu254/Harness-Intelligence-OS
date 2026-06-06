# Validation

## Proof Strategy

Prove the adapter as a grounded-context producer and the existing US-024 command
as the trust boundary. Tests must distinguish provider/session unavailability,
uncited summaries, malformed artifacts, source-provenance failures, and valid
grounded evidence.

Implementation must exercise at least one real NotebookLM-compatible provider
or export path before claiming completion. Planning alone must not pass the
story gate.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Provider response mapping, source hashes, citations, claims, unavailable and error status mapping |
| Integration | Generated artifact validates against US-023 and passes through US-024 ingest without direct SQLite access |
| E2E | Adapter produces artifact, ingest records pass, context pack renders grounded context, story gate accepts required NotebookLM proof |
| Platform | Path, process/session, exit-code, and JSON behavior on supported adapter platforms |
| Failure | Missing provider/session/source is inconclusive; uncited claims, malformed output, missing provenance, and hash mismatch are fail |
| Logs/Audit | No credentials in output; raw response reference, artifact, SHA256, ingest report, SQLite summary, and Detailed trace are inspectable |

## Fixtures

- Passing grounded brief with at least one source and cited claim.
- Passing brief with constraints, open questions, and affected docs.
- Provider unavailable or session expired response.
- Source unavailable response.
- Summary with no citations.
- Claim citation pointing to an unknown source.
- Source hash mismatch.
- Malformed or incomplete provider response.

## Commands

Exact adapter commands are added after provider discovery. Baseline validation:

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-mcp-artifact-contracts.py
harness-cli context ingest --story US-026 --source notebooklm --file <generated-artifact>
harness-cli context --story US-026
harness-cli arch-check --story US-026
harness-cli story verify US-026
```

## Acceptance Criteria

- NotebookLM-compatible provider contract is documented and accepted.
- Authentication/session behavior is documented without storing secrets.
- Raw response is captured or referenced safely.
- The adapter writes a schema-valid `notebooklm-brief` artifact.
- Passing artifacts contain required source provenance and citations.
- Uncited summaries or claims fail validation and cannot satisfy governance.
- Provider unavailable, permission denied, timeout, source unavailable, or
  insufficient evidence is inconclusive and cannot satisfy governance.
- The adapter never writes directly to Harness SQLite.
- The generated artifact passes through US-024 context ingest.
- Passing evidence updates grounded context and appears in the context pack.
- An explicitly configured NotebookLM requirement passes the story governance
  gate only after successful ingest.
- CodeGraph, auto-intake defaults, release/tag, and installer pin remain
  unchanged.

## Acceptance Evidence

Evidence is added after implementation and verification.

## Planning Evidence

- Intake: `15`.
- Story: `US-026`, High-Risk, NotebookLM ingest proof required.
- GitHub issue: `#4`.
- Milestone: `HI-OS v0.4.0: Real MCP Adapter Layer`.
- Context pack: `.harness/context/US-026-context.md`.
- Architecture check: passed during story creation.
- Planning trace: `#19`, Detailed `3/3`.
- Decision 0010 remains the governing file-based boundary.
- Provider contract is intentionally pending.
