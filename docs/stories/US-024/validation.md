# Validation

## Proof Strategy

Use temporary repositories and databases to validate migration, typed artifact
contracts, result files, durable summaries, intake mapping, context pack
rendering, and explicit governance requirements.

## Test Plan

| Layer | Cases |
| --- | --- |
| Unit | Source parsing, schema version/type/story checks, provenance, citations, pass/fail/inconclusive mapping |
| Integration | Migration 006, SQLite summary, intake updates only on pass, context pack summaries, explicit story gate |
| E2E | CLI help and local artifact ingestion for both sources |
| Platform | Windows repo-local release binary smoke after build |
| Performance | Small local JSON files; no dedicated benchmark |
| Logs/Audit | Result JSON, SHA256, SQLite row, trace, and gate result |

## Fixtures

- Passing CodeGraph artifact.
- Passing NotebookLM artifact.
- Missing provenance artifact.
- Declared provider failure artifact.
- Provider unavailable artifact.
- Story/source/type mismatch artifacts.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release --package harness-cli
python scripts/verify-mcp-artifact-contracts.py
target/release/harness-cli.exe arch-check --story US-024
target/release/harness-cli.exe story verify US-024
```

## Acceptance Evidence

- `cargo fmt --check`: passed.
- `cargo test --workspace`: 30 passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo build --release --package harness-cli`: passed on Windows.
- `python scripts/verify-mcp-artifact-contracts.py`: passed.
- Migration 006 applied from schema version 5 to 6.
- CodeGraph E2E ingest: pass with SHA256 and schema-valid result JSON.
- NotebookLM E2E ingest: pass with SHA256 and schema-valid result JSON.
- Invalid CodeGraph `change_kind` E2E attempt: fail and durable audit row.
- Missing provenance integration case: fail without intake mutation.
- Provider unavailable integration case: inconclusive and not governance
  eligible.
- Context pack includes the latest validated summary for both sources.
- Explicit CodeGraph and NotebookLM story requirements are enforced.
- PowerShell installer parser and `bash -n` passed with migration 006 in the
  payload.
- Architecture check passed.
- Story verification passed with 30 tests.
- Story governance gate passed with both explicit ingest requirements enabled.
- Trace `#15` achieved Detailed `3/3`.

## Durable Evidence

- Intake: `13`.
- Story: `US-024`.
- GitHub issue: `#2`.
- Operational reports:
  - `.harness/context/US-024-codegraph-ingest-result.json`
  - `.harness/context/US-024-notebooklm-ingest-result.json`
- Context pack: `.harness/context/US-024-context.md`.
- SQLite table: `context_ingest`.
