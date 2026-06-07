# US-047 Execution Plan

1. Open GitHub issue #24 and record high-risk story state.
2. Run `rg` audit for legacy names, runtime/generated files, and mock artifacts.
3. Write `docs/stories/US-047/cleanup-ledger.md`.
4. Apply only the cleanup actions supported by the ledger.
5. Update docs/verifiers if primary identity wording changes.
6. Generate context and architecture evidence.
7. Run full validation:

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-adoption-docs.py
python scripts/verify-governance-report-schema.py
python scripts/verify-mcp-artifact-contracts.py
python scripts/verify-friction-taxonomy.py
harness-cli identity
harness-cli context --story US-047
harness-cli arch-check --story US-047
harness-cli story verify US-047
```

8. Record Detailed trace and close issue #24.
