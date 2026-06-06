# Validation

## Proof Strategy

Validate each schema against Draft 2020-12, prove representative passing
artifacts, and prove critical invalid states are rejected. Existing Rust tests
must remain green because US-023 documents a future boundary without changing
current CLI behavior.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Schemas are valid; pass artifacts validate; missing provenance, citations, unavailable state, and governance false/true mismatches fail |
| Integration | Artifact fields map unambiguously to current intake and context pack fields |
| E2E | Not applicable until US-024 implements `context ingest` |
| Platform | JSON contracts are platform-neutral |
| Performance | Not applicable for contract design |
| Logs/Audit | Artifact ids, hashes, invocation ids, claim ids, status, and ingest eligibility are specified |

## Fixtures

`scripts/verify-mcp-artifact-contracts.py` creates deterministic in-memory
fixtures for:

- Passing CodeGraph impact.
- Passing NotebookLM grounded brief.
- Passing context ingest result.
- Inconclusive CodeGraph artifact without unavailable evidence.
- NotebookLM claim without citations.
- Inconclusive ingest result incorrectly marked governance-eligible.

## Commands

```text
python scripts/verify-mcp-artifact-contracts.py
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
scripts/bin/harness-cli arch-check --story US-023
scripts/bin/harness-cli story verify US-023
```

## Acceptance Evidence

- Draft 2020-12 meta-schema checks passed for all three schemas.
- Passing CodeGraph, NotebookLM, and context ingest fixtures validated.
- Valid deterministic failure and unavailable-source fixtures validated.
- Missing provenance, missing citations, missing unavailable details, and
  governance-eligible inconclusive results were rejected.
- Existing Rust format, test, and clippy checks passed with 27 tests.
- Decision 0010 verification and architecture check passed.
- Context pack generated at `.harness/context/US-023-context.md`.
- GitHub milestone
  `HI-OS v0.4.0: Real MCP Adapter Layer` and issue `#1` track the initiative
  and US-023.
- Trace #13 and final edge-case Trace #14 achieved Detailed 3/3; the final
  story governance gate passed after unavailable-source validation.
