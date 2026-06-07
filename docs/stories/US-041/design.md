# Design

## Domain Model

The example uses a fictional local story:

- `US-EXAMPLE`: Add contributor onboarding note.

The example is docs-only so users can focus on the governance loop rather than
an application framework.

## Application Flow

The documented agent flow is:

```text
read operating context
  -> intake
  -> story add
  -> optional provider context
  -> context pack
  -> smallest implementation
  -> validation
  -> proof flags
  -> trace
  -> story verify
  -> governance report/dashboard
```

## Interface Contract

No CLI command changes. The example uses existing commands:

- `intake`;
- `story add`;
- `codegraph impact`;
- `notebooklm brief`;
- `context`;
- `arch-check`;
- `story update`;
- `trace`;
- `story verify`;
- `governance report`;
- `governance dashboard`.

`python scripts/verify-adoption-docs.py` verifies the example contains the
required command and troubleshooting sections.

## Data Model

No SQLite migration is added. The example uses runtime state only.

## UI / Platform Impact

The example includes Bash and Windows PowerShell command shapes where command
syntax differs.

## Observability

The example teaches users to produce:

- context pack;
- validation command output;
- detailed trace;
- story governance gate result;
- governance report JSON;
- static governance dashboard HTML.

## Alternatives Considered

1. Use a real app feature example.
   - Rejected because HI-OS has no bundled app stack and adoption docs should
     not imply one.
2. Put the full example in README.
   - Rejected because US-040 intentionally kept README to five-minute
     orientation.
3. Require live CodeGraph and NotebookLM proof.
   - Rejected because US-041 is adoption documentation, not provider setup.

