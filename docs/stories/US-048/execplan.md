# US-048 Execution Plan

1. Record issue #25, intake, story, and Decision 0014.
2. Add the production include/exclude manifest.
3. Implement canonical Python builder and verifier.
4. Add PowerShell and Bash entrypoints.
5. Update README, scripts docs, adoption docs, and CI syntax validation.
6. Build the v0.7.0 production ZIP and SHA256.
7. Verify archive contents and deterministic rebuild.
8. Extract the archive outside the source repo.
9. Run the packaged installer into a clean target with a separate trusted CLI.
10. Run CLI, governance report/dashboard, full verifiers, context, architecture,
    trace, and story gate.

## Validation Commands

```text
python scripts/build-production-payload.py --version 0.7.0
python scripts/verify-production-payload.py --version 0.7.0 --source-check
scripts/build-production-payload.sh --version 0.7.0
scripts/build-production-payload.ps1 -Version 0.7.0
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
python scripts/verify-adoption-docs.py
python scripts/verify-governance-report-schema.py
python scripts/verify-mcp-artifact-contracts.py
python scripts/verify-friction-taxonomy.py
harness-cli context --story US-048
harness-cli arch-check --story US-048
harness-cli story verify US-048
```
