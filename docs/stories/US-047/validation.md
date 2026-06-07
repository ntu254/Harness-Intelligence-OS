# US-047 Validation

US-047 proves legacy cleanup is intentional and does not remove durable HI-OS
evidence.

## Acceptance Criteria

- Cleanup ledger exists.
- KEEP / MIGRATE / ARCHIVE / DELETE classifications are documented.
- Primary docs use HI-OS identity.
- Historical evidence is preserved.
- No runtime/generated artifacts are newly tracked.
- Full validation passes.
- Detailed trace records cleanup decisions.

## Evidence

- GitHub issue #24 opened under HI-OS v0.7.0 Adoption Ready.
- Intake #35 recorded for US-047.
- Cleanup ledger added at `docs/stories/US-047/cleanup-ledger.md`.
- Root historical phase plans archived:
  - `PHASE2.md` -> `docs/archive/phases/PHASE2.md`
  - `PHASE3.md` -> `docs/archive/phases/PHASE3.md`
  - `PHASE4.md` -> `docs/archive/phases/PHASE4.md`
- Early HI-OS technical spec archived:
  - `harness_intelligence_os_spec.md` ->
    `docs/archive/specs/harness_intelligence_os_spec.md`
- `docs/archive/README.md` marks archived files as provenance, not current
  operating policy.
- Primary docs migrated from stale Harness v0/repository-harness framing to
  HI-OS wording.
- Phase plan references now point to `docs/archive/phases/*`.
- No tracked runtime/generated/mock artifact was deleted; audit found ignored
  runtime/build outputs only.
- `.gitignore` continues to ignore `harness.db`, `.harness/`, `.codegraph/`,
  `target/`, `dist/`, and downloaded CLI binaries.
- `target/release/harness-cli.exe identity` passed.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed with 50 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `python scripts/verify-adoption-docs.py` passed.
- `python scripts/verify-governance-report-schema.py` passed.
- `python scripts/verify-mcp-artifact-contracts.py` passed.
- `python scripts/verify-friction-taxonomy.py` passed.
- `bash -n scripts/install-harness.sh` passed.
- PowerShell parser accepted `scripts/install-harness.ps1`.
- `git diff --check` passed.
- `target/release/harness-cli.exe context --story US-047` generated
  `.harness/context/US-047-context.md`.
- `target/release/harness-cli.exe arch-check --story US-047` passed.
- Trace #45 recorded Detailed 3/3 and met the high-risk trace requirement.
- `target/release/harness-cli.exe story verify US-047` passed mechanical
  verification and governance gate.
- `target/release/harness-cli.exe governance report --output
  .harness/reports/US-047-governance-report.json` passed.
- `python scripts/verify-governance-report-schema.py
  .harness/reports/US-047-governance-report.json` passed.
- `target/release/harness-cli.exe governance dashboard --report
  .harness/reports/US-047-governance-report.json --output
  .harness/dashboard/US-047-index.html` passed.
- Final dashboard shows `Harness Intelligence OS`, US-047 gate `pass`, and
  maturity `trusted (93)`.
