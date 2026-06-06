# Validation

## Acceptance Criteria

- CodeGraph pass evidence can influence auto-intake risk flags and code impact
  summary.
- NotebookLM pass evidence can influence auto-intake grounded context.
- Inconclusive evidence never becomes pass evidence.
- Failed evidence never becomes pass evidence.
- Missing required evidence remains blocked by story governance policy.
- Context pack clearly displays pass/fail/inconclusive ingest state.
- No provider calls or direct provider-to-SQLite writes are introduced.

## Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
harness-cli context --story US-027
harness-cli arch-check --story US-027
harness-cli story verify US-027
```

## Evidence

- `harness-cli intake --auto --story <id>` now prefers latest passing
  CodeGraph and NotebookLM `context_ingest` reports before ad hoc fallback
  inputs.
- `AutoIntakeEvidence` reads mapped context from the latest source row only
  when that row is `pass`.
- Failed and inconclusive evidence are ignored for auto-intake seeding.
- Context pack rendering includes durable ingest diagnostics for non-passing
  evidence.
- Tests added:
  - passing CodeGraph and NotebookLM mapped context is readable for auto intake.
  - latest inconclusive NotebookLM evidence does not promote an older pass.
  - mapped context merge seeds risk flags, affected docs, code impact, and
    grounded context.
- Validation run:
  - `cargo fmt --check`
  - `cargo test --workspace` (`41` tests)
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `python scripts/verify-mcp-artifact-contracts.py`
  - `cargo build --release -p harness-cli`
  - `harness-cli arch-check --story US-027`
  - `harness-cli context --story US-027`
  - `harness-cli story verify US-027`
- Trace: `#22`, Detailed `3/3`.
- Story verification: `pass`.
- Story governance gate: `pass`.

## Planning Evidence

- Intake: `#16`.
- Story: `US-027`, High-Risk.
- GitHub issue: `#5`.
- Milestone: `HI-OS v0.4.0: Real MCP Adapter Layer`.
