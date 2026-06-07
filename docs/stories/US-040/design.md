# Design

## Domain Model

README is treated as the adoption landing page. It must introduce these
concepts without requiring readers to know Harness history:

- HI-OS: the repository operating layer for agents.
- Quickstart: install, initialize, create work item, verify, dashboard.
- Trust evidence: release verify, governance dashboard, provider artifacts.
- Runtime evidence: local ignored state such as `harness.db` and `.harness/`.

## Application Flow

The README reading flow is:

```text
what HI-OS is
  -> why it exists
  -> five-minute quickstart
  -> core workflow
  -> release trust
  -> dashboard trust
  -> CodeGraph/NotebookLM context
  -> repo map and next docs
```

## Interface Contract

No CLI behavior changes. The documentation contract is enforced by:

```text
python scripts/verify-adoption-docs.py
```

The verifier checks README sections, required command phrases, release trust
language, provider behavior, and the clean clone walkthrough link.

## Data Model

No SQLite migration is added. README explains that runtime evidence is local
and ignored.

## UI / Platform Impact

README includes Bash and Windows PowerShell install and release-verify command
shapes.

## Observability

The README points users toward:

- `story verify` for governance gate status;
- `release verify` for trusted distribution evidence;
- `governance report` and `governance dashboard` for auditable summary.

## Alternatives Considered

1. Keep the old README and only add a larger quickstart section.
   - Rejected because the first screen still made HI-OS feel secondary.
2. Move all installer details out of README.
   - Rejected because first-time users need at least one copy/paste install
     path.
3. Add command cookbook content directly to README.
   - Rejected because US-044 owns the cookbook and README should remain a
     five-minute orientation.

