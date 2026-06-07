# Design

## Domain Model

The adoption surface has three user concepts:

- Clean clone: a repository checkout without local runtime state.
- Runtime evidence: ignored files such as `harness.db`, `.harness/`, `target/`,
  and `dist/`.
- Walkthrough story: a local demo story that proves the governance path without
  becoming product contract.

## Application Flow

The documented flow is:

```text
git clone
  -> cargo build --package harness-cli --release
  -> harness-cli init
  -> harness-cli import brownfield
  -> harness-cli intake/story add for US-DEMO
  -> harness-cli context
  -> harness-cli arch-check
  -> harness-cli trace
  -> harness-cli story verify
  -> governance report/dashboard
  -> optional release verify
```

## Interface Contract

US-039 does not add a new CLI command. It adds a verifier script:

```text
python scripts/verify-adoption-docs.py
```

The verifier checks that the walkthrough and index links contain the required
first-run commands and trust-boundary notes.

## Data Model

No SQLite migration is added. The walkthrough explains that `harness.db` is
local runtime state and ignored.

## UI / Platform Impact

The walkthrough includes POSIX shell and Windows PowerShell command shapes.

## Observability

The walkthrough teaches users to generate:

- `.harness/context/US-DEMO-context.md`;
- `.harness/reports/US-DEMO-governance-report.json`;
- `.harness/dashboard/US-DEMO-index.html`;
- a trace linked to `US-DEMO`.

## Alternatives Considered

1. Rewrite the full README first.
   - Rejected for US-039 because README rewrite is a separate adoption story.
2. Use only public installer commands.
   - Rejected because a source clone does not use `scripts/bin/harness-cli`
     until the CLI is built or installed.
3. Pretend clean clone has the maintainer's local story matrix.
   - Rejected because durable history is intentionally local and ignored.

