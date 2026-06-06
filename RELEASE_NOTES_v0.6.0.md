# HI-OS v0.6.0: Governance Dashboard & Maturity

## Added

- Governance report contract and Draft 2020-12 schema.
- `harness-cli governance report` for static JSON governance snapshots.
- Deterministic governance maturity summary:
  - score;
  - level;
  - gate and validation pass percentages;
  - release verification flag;
  - open governance gaps;
  - explanatory notes.
- `harness-cli governance dashboard` for standalone static HTML export.
- Installer payload coverage for governance report docs, schema, decision, and
  verifier.

## Verified

- 49 Rust tests passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- Governance report schema verifier passed.
- Friction taxonomy verifier passed.
- MCP artifact contract verifier passed.
- Governance report and dashboard smoke commands passed.
- Story governance gates passed for US-034 through US-038.

## Notes

- Dashboard export is static HTML only. It uses no external assets, no live
  server, and no scripts.
- Report and dashboard generation are read-only except for writing requested
  output artifacts.
- v0.5.0 tag and release assets remain unchanged.
