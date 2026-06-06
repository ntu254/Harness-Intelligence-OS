# Validation

## Acceptance Criteria

- v0.4 release notes describe US-023 through US-027.
- CLI help/smoke covers `context ingest`, `codegraph impact`,
  `notebooklm brief`, and auto-intake evidence defaults.
- `cargo fmt --check` passes.
- `cargo test --workspace` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `python scripts/verify-mcp-artifact-contracts.py` passes.
- Five platform binaries and five SHA256 assets are built.
- GitHub release `harness-cli-v0.4.0` is published.
- `harness-cli release verify --version 0.4.0 --story US-028` passes.
- CodeGraph evidence example is valid and ingested.
- NotebookLM provider pass evidence is valid, or provider unavailable is
  recorded as honest inconclusive without becoming pass.
- Context pack is generated.
- Trace is Detailed `3/3`.
- Story governance gate passes only after release proof exists.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-mcp-artifact-contracts.py
harness-cli context --story US-028
harness-cli release verify --version 0.4.0 --story US-028
harness-cli story verify US-028
```

## Evidence

Evidence is added after implementation and release verification.

## Planning Evidence

- Intake: `#17`.
- Story: `US-028`, High-Risk.
- GitHub issue: `#6`.
- Release proof required: yes.
- Milestone: `HI-OS v0.4.0: Real MCP Adapter Layer`.
- Architecture check: pass.
- Context pack: `.harness/context/US-028-context.md`.
- Planning trace: `#23`, Detailed `3/3`.
