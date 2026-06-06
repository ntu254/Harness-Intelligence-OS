# Design

## Domain Model

Architecture rules contain a layer name, source path, optional file filter, and
forbidden import path segments. A story gate returns pass/fail plus missing
evidence labels.

## Application Flow

`arch-check` loads TOML rules, scans source imports, prints violations, and
optionally stores the result on a story. `story verify` runs mechanical proof,
then evaluates durable evidence and exits non-zero on any missing item.

## Interface Contract

```text
harness-cli arch-check [--config <path>] [--story <id>]
harness-cli story verify <id>
```

## Data Model

Migration `004-verification-gate.sql` records automated intake provenance,
architecture timestamps, and story gate results.

## UI / Platform Impact

Terminal output lists each architecture violation or each missing gate item.

## Observability

Architecture and gate timestamps/results are persisted on the story. Task
execution remains visible through linked traces.

## Alternatives Considered

See decision `0007-hi-os-verification-gate`.
