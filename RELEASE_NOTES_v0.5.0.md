# HI-OS v0.5.0: Harness Learning Loop

## Added

- Friction taxonomy and Draft 2020-12 `friction-event` schema.
- `harness-cli friction add` for structured friction capture.
- `harness-cli query friction-events` for durable structured friction review.
- Context-pack rendering for story-linked structured friction.
- `harness-cli backlog suggest` for read-only backlog suggestions from
  structured friction.
- `harness-cli rules suggest` for read-only rule-improvement proposals from
  structured friction.
- SQLite migration `007-friction-events.sql`.
- Installer payload coverage for v0.4 MCP schemas and v0.5 friction contracts.

## Verified

- 46 Rust tests passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- MCP artifact contract verifier passed.
- Friction taxonomy verifier passed.
- Installer PowerShell and shell syntax checks passed.
- Story governance gates passed for US-029 through US-033.

## Notes

- v0.4.0 tag and release assets remain unchanged.
- `backlog suggest` and `rules suggest` are read-only. Backlog rows, decisions,
  docs, schemas, and policy files still require explicit human-reviewed edits.
- NotebookLM live-provider proof remains external/session dependent; provider
  unavailability remains inconclusive and never pass.
