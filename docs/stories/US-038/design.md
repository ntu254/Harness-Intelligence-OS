# Design

## Release Flow

```text
version bump
  -> installer pin
  -> release notes
  -> local validation
  -> tag harness-cli-v0.6.0
  -> GitHub release workflow
  -> release verify 0.6.0
  -> governance report/dashboard evidence
  -> story gate
```

## Installer Payload

The installer payload includes v0.6 governance report artifacts:

- `docs/GOVERNANCE_REPORT.md`
- `docs/decisions/0012-governance-report-schema.md`
- `docs/schemas/governance-report.schema.json`
- `scripts/verify-governance-report-schema.py`

## Guardrails

- Older tags and release assets remain immutable.
- Release verification must pass before story gate can pass.
- Governance report and dashboard generation remain read-only.
- No migration is added for v0.6 release hardening.

## Evidence

US-038 requires:

- local Rust validation;
- schema verifiers;
- installer syntax checks;
- release build and CLI version smoke;
- public release workflow pass;
- public release verification pass;
- governance report/dashboard smoke;
- Detailed trace.
