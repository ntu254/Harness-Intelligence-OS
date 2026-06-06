# Design

## Interface

```text
harness-cli governance dashboard \
  --report .harness/reports/governance-report.json \
  --output .harness/dashboard/index.html
```

Defaults:

- report: `.harness/reports/governance-report.json`
- output: `.harness/dashboard/index.html`

## Rendering

The dashboard reads a governance report JSON artifact and renders:

- maturity level and score;
- story count;
- gate pass/fail/not-run counts;
- release verification result;
- friction event count;
- open governance gap count;
- maturity notes;
- story evidence rows.

## Guardrails

- Dashboard export writes only the requested HTML file.
- Dashboard export does not query or mutate SQLite.
- Dashboard export does not run story verification.
- Dashboard export does not run release verification.
- Dashboard export uses no external assets.
- Dashboard export uses no script execution.

## Failure Semantics

If the report path is missing, unreadable, unparsable, or not a
`governance-report`, export fails.
