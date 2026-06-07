# US-046 Validation

US-046 proves HI-OS has a tracked sovereign identity visible through CLI,
governance report JSON, dashboard HTML, and release verification defaults.

## Acceptance Criteria

- `hios.toml` exists and defines HI-OS identity.
- `harness-cli identity` reports product name, short name, repository, and
  default release origin.
- Governance report JSON includes the `identity` object.
- Governance dashboard displays HI-OS identity.
- `release verify` default origin is aligned with
  `ntu254/Harness-Intelligence-OS`.
- Decision 0013 is accepted and verified.
- README/docs use HI-OS as the primary identity.
- No release/tag/installer pin change.
- Detailed trace records identity boundary behavior.

## Evidence

- `hios.toml` added with:
  - product: `Harness Intelligence OS`
  - short name: `HI-OS`
  - repository: `ntu254/Harness-Intelligence-OS`
  - default release origin: `ntu254/Harness-Intelligence-OS`
- `target/release/harness-cli.exe identity` passed and printed the tracked
  product identity.
- `target/release/harness-cli.exe release verify --version 0.6.0 --story US-046`
  passed using default origin `ntu254/Harness-Intelligence-OS`.
- `.harness/release/US-046-release-verify.json` records 10 checked assets,
  download pass, checksum pass, version pass, and smoke install pass.
- `target/release/harness-cli.exe governance report --output
  .harness/reports/US-046-governance-report.json` passed.
- `python scripts/verify-governance-report-schema.py
  .harness/reports/US-046-governance-report.json` passed.
- Generated governance report schema version is `1.1.0` and includes
  `identity.product_name = Harness Intelligence OS`.
- `target/release/harness-cli.exe governance dashboard --report
  .harness/reports/US-046-governance-report.json --output
  .harness/dashboard/US-046-index.html` passed.
- Dashboard HTML displays `Harness Intelligence OS`, `HI-OS`, and default
  release origin `ntu254/Harness-Intelligence-OS`.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed with 50 tests.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `python scripts/verify-governance-report-schema.py` passed.
- `python scripts/verify-adoption-docs.py` passed.
- `target/release/harness-cli.exe decision verify
  0013-hi-os-sovereign-identity` passed.
- `target/release/harness-cli.exe context --story US-046` generated
  `.harness/context/US-046-context.md`.
- `target/release/harness-cli.exe arch-check --story US-046` passed.
- Trace #44 recorded Detailed 3/3.
- `target/release/harness-cli.exe story verify US-046` passed mechanical
  verification and governance gate.
- Final US-046 governance dashboard shows maturity `trusted (93)` and gate pass
  count `26`.

## Implementation Notes

- Added `harness-cli identity`.
- Added required `identity` object to governance report schema v1.1.0.
- Governance dashboard renders tracked identity separately from Git origin.
- `release verify` checks `hios.toml` identity alignment with
  `harness-release.toml` before running release checks.
- Installer payload now includes `hios.toml`.
- No release tag, installer pin, provider behavior, or legacy cleanup was
  changed.
