# Validation

## Proof Strategy

Prove the adapter as a producer and the existing US-024 command as the trust
boundary. Tests must distinguish adapter mapping failures, provider
unavailability, artifact contract failures, and ingest governance outcomes.

At least one verification path must exercise the real configured
CodeGraph-compatible invocation. Deterministic fixtures cover repeatable
mapping and failure semantics.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Provider response mapping, provenance, hashes, status mapping, affected files, dependency edges, risk flags, claims |
| Integration | Generated artifact validates against US-023 and passes through US-024 ingest without direct SQLite access |
| E2E | Adapter produces artifact, ingest records pass, context pack renders evidence, story gate accepts required CodeGraph proof |
| Platform | Path, process, exit-code, and JSON behavior on supported adapter platforms |
| Failure | Missing executable, missing index, or non-zero provider exit is inconclusive; malformed output or provenance mismatch is fail; neither satisfies governance |
| Logs/Audit | No credentials in output; artifact, SHA256, ingest report, SQLite summary, and Detailed trace are inspectable |

## Fixtures

- Passing CodeGraph-compatible response with affected files and claims.
- Passing response with dependency edges and multiple risk flags.
- Provider unavailable response or invocation failure.
- Deterministic provider failure.
- Malformed or incomplete provider response.
- Repository revision and hashed input provenance.

## Commands

Validation commands:

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-mcp-artifact-contracts.py
codegraph status --json
harness-cli codegraph impact --story US-025 --mode changed-files --changed-files .harness/context/US-025-changed-files.txt --depth 5
harness-cli codegraph impact --story US-025 --mode symbol --symbol validate_context_artifact --depth 2
harness-cli context --story US-025
harness-cli arch-check --story US-025
harness-cli story verify US-025
```

## Acceptance Criteria

- A real CodeGraph-compatible adapter command exists.
- The adapter writes a schema-valid `codegraph-impact` artifact.
- Passing artifacts contain required provenance, SHA256-backed inputs, impact
  evidence, and grounded claims.
- Provider unavailability creates `inconclusive`, exits non-zero, and cannot
  satisfy governance.
- Missing executable, missing index, or non-zero provider exit creates
  `inconclusive`, exits non-zero, and cannot satisfy governance.
- Malformed provider JSON or invalid provenance creates `fail`, exits non-zero,
  and cannot satisfy governance.
- The adapter never writes directly to Harness SQLite.
- The generated artifact passes through US-024 context ingest.
- Passing evidence updates the linked intake and appears in the context pack.
- An explicitly configured CodeGraph requirement passes the story governance
  gate only after successful ingest.
- NotebookLM, auto-intake defaults, release/tag, and installer pin remain
  unchanged.

## Acceptance Evidence

- CodeGraph CLI `0.9.9` installed without authentication.
- `codegraph init .` indexed 7 files, 515 nodes, and 1,567 edges.
- Changed-files provider JSON matched the observed CodeGraph contract and
  normalized into a schema-valid passing US-023 artifact.
- Symbol impact mode produced and ingested a schema-valid passing artifact.
- Missing executable produced `inconclusive` and did not satisfy governance.
- Invalid provider stdout produced `fail` and did not satisfy governance.
- Missing provenance produced `fail`.
- Local provenance SHA256 mismatch produced `fail`.
- Passing adapter output was ingested through US-024 and rendered in the
  context pack.
- SQLite retained inconclusive, fail, and pass attempts as separate audit
  summaries.
- `cargo test --workspace`: 34 passed.
- `cargo fmt --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Release build passed on Windows.
- Generated CodeGraph and ingest artifacts validated against their Draft
  2020-12 schemas.
- Architecture check passed.
- PowerShell and Bash installer syntax checks passed.
- Story verification and the explicit CodeGraph governance gate passed.
- Implementation trace `#18`: Detailed `3/3`.

## Planning Evidence

- Intake: `14`.
- Story: `US-025`, High-Risk, CodeGraph ingest proof required.
- GitHub issue: `#3`.
- Milestone: `HI-OS v0.4.0: Real MCP Adapter Layer`.
- Context pack: `.harness/context/US-025-context.md`.
- Architecture check: passed during story creation.
- Planning trace: `#16`, Detailed `3/3`.
- Decision 0010 remains the governing file-based boundary; no new decision or
  schema migration was introduced during planning.
- Design review: accepted on 2026-06-07.
- Provider discovery checked repository files, PATH commands, environment
  variables, MCP resources/templates, and deferred tools. No callable
  CodeGraph-compatible provider was available.
- CodeGraph CLI `0.9.9` supplied and verified the real provider boundary.
- Design review trace: `#17`, Detailed `3/3`, outcome `partial`.
