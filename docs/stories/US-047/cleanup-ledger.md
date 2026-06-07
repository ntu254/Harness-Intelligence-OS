# US-047 Cleanup Ledger

This ledger records the legacy cleanup classification.

## KEEP

- `docs/decisions/*`: durable decision history. Older `Harness v0` wording is
  historical evidence, not current primary identity.
- `docs/stories/*`: story packets and validation evidence. Historical stories
  retain the wording that was true when the work was done.
- `docs/schemas/*`: active artifact/report contracts.
- `scripts/schema/*`: migration files are durable schema history. Header
  comments were migrated to HI-OS wording, but migration files remain in place.
- `docs/HARNESS_COMPONENTS.md`: active component taxonomy. Migrated primary
  wording from repository-harness to HI-OS.
- `docs/HARNESS_MATURITY.md`: active maturity ladder. Migrated primary wording
  from repository-harness to HI-OS.
- `docs/TRACE_SPEC.md`: active trace contract. Phase references now point to
  archived phase plans.
- Installer legacy AGENTS detector string:
  `This repository is in Harness v0. There is no product implementation yet.`
  This exact text remains because it detects old generated files safely.
- Ignored runtime files and directories in the working copy:
  `harness.db`, `.harness/`, `.codegraph/`, `target/`, and `dist/`.

## MIGRATE

- `README.md`: added `docs/archive/` to repository map.
- `docs/README.md`: replaced stale Harness v0 current-state wording with
  active HI-OS wording.
- `docs/product/README.md`: removed Harness v0 framing from the empty product
  docs explanation.
- `docs/demo/README.md`: changed the example framing from Harness v0 to HI-OS.
- `scripts/README.md`: changed user-facing installer/import wording to HI-OS.
- `scripts/install-harness.sh`: changed user-facing help/prompt wording to
  HI-OS while preserving the old generated-file detector.
- `docs/HARNESS.md`: migrated scope/spec lifecycle headings from Harness v0 to
  HI-OS.
- `docs/HARNESS_COMPONENTS.md`, `docs/TRACE_SPEC.md`, and historical story
  references: updated phase plan paths to `docs/archive/phases/*`.
- `crates/harness-cli/src/domain.rs` and
  `crates/harness-cli/src/infrastructure.rs`: updated test/example strings
  that referenced root phase files.
- `.gitignore`: fixed mojibake in the runtime-artifact comment.

## ARCHIVE

- `PHASE2.md` -> `docs/archive/phases/PHASE2.md`.
- `PHASE3.md` -> `docs/archive/phases/PHASE3.md`.
- `PHASE4.md` -> `docs/archive/phases/PHASE4.md`.
- `harness_intelligence_os_spec.md` ->
  `docs/archive/specs/harness_intelligence_os_spec.md`.
- Added `docs/archive/README.md` to mark archived documents as provenance, not
  current operating policy.

## DELETE

- No tracked mock/runtime/generated artifact was deleted. Audit found only
  source templates and ignored runtime/build outputs.
