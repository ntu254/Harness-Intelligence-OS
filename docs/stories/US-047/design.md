# US-047 Design: Intentional Legacy Cleanup

US-047 is a cleanup story, but cleanup still needs governance. The rule is:
do not delete evidence just because the wording is old.

## Classification Ledger

Every legacy artifact found during audit is classified as one of:

- `KEEP`: historical or operational evidence that should remain in place.
- `MIGRATE`: primary docs or user-facing files that should use HI-OS identity.
- `ARCHIVE`: useful historical docs that should move under `docs/archive/`.
- `DELETE`: tracked mock/runtime/generated files with no durable value.

## Preservation Rules

Keep:

- decisions;
- story packets;
- schemas;
- release notes;
- validation docs;
- trace-linked evidence descriptions;
- architecture and context rules.

Archive instead of delete when a document explains why HI-OS evolved from
repository-harness.

Delete only when a tracked artifact is clearly generated, mock-only, obsolete,
and not linked as story evidence.

## Validation Surface

The cleanup must preserve:

- adoption docs verifier;
- governance report schema verifier;
- Rust tests, fmt, clippy;
- context pack generation;
- architecture check;
- story governance gate.
