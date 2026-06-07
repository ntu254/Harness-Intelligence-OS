# US-046 Execution Plan

1. Open GitHub issue #23 under HI-OS v0.7.0 Adoption Ready.
2. Record high-risk intake and story row.
3. Add Decision 0013 for sovereign identity.
4. Add root `hios.toml`.
5. Implement identity config loading and `harness-cli identity`.
6. Add identity to governance report JSON and dashboard HTML.
7. Validate release default origin alignment against identity.
8. Update docs, schema, verifier, and installer payload.
9. Run tests, schema verifiers, report/dashboard smoke, context, architecture,
   trace score, and story gate.

## Validation Commands

```text
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-governance-report-schema.py
python scripts/verify-adoption-docs.py
harness-cli identity
harness-cli governance report --output .harness/reports/US-046-governance-report.json
python scripts/verify-governance-report-schema.py .harness/reports/US-046-governance-report.json
harness-cli governance dashboard --report .harness/reports/US-046-governance-report.json --output .harness/dashboard/US-046-index.html
harness-cli context --story US-046
harness-cli arch-check --story US-046
harness-cli story verify US-046
```
