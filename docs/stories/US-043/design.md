# Design

## Domain Model

Troubleshooting is organized by failing proof surface:

- installer;
- release verification;
- CodeGraph provider;
- NotebookLM provider;
- story governance gate;
- governance report/dashboard.

Every section preserves the same semantics:

- trust failures are `fail`;
- unavailable dependencies are `inconclusive`;
- neither counts as `pass`.

## Application Flow

```text
observe symptom
  -> identify failing proof surface
  -> inspect runtime evidence
  -> rerun exact command
  -> fix missing setup or proof
  -> rerun story verify
```

## Interface Contract

No CLI changes. The guide references existing commands and is checked by:

```text
python scripts/verify-adoption-docs.py
```

## Data Model

No SQLite migration is added.

## UI / Platform Impact

The guide includes Bash and Windows PowerShell forms for the main recovery
commands.

## Observability

The guide points users to `.harness/`, `harness.db`, story matrix, JSON reports,
and traces.

## Alternatives Considered

1. Put troubleshooting directly into README.
   - Rejected because README should remain a five-minute orientation.
2. Wait for the command cookbook.
   - Rejected because troubleshooting explains failure interpretation, not just
     command syntax.

