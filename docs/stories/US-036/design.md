# Design

## Inputs

The maturity summary is derived from existing governance report sections:

- `gate_summary`
- `validation_summary`
- `release_summary`
- `friction_summary`

No new SQLite table is added.

## Scoring

The score is integer-only and bounded from 0 to 100:

| Component | Max |
| --- | ---: |
| Story gate pass rate | 40 |
| Validation command pass rate | 25 |
| Release verification | 20 |
| High-severity friction pressure | 15 |

Release verification contributes:

- `pass`: 20
- `inconclusive`: 10
- `fail` or `none`: 0

Each high-severity friction event subtracts 5 points from the friction
component, capped at 15.

## Levels

| Score | Level |
| ---: | --- |
| 85-100 | `trusted` |
| 70-84 | `managed` |
| 50-69 | `developing` |
| 0-49 | `early` |

## Guardrails

- Maturity scoring is read-only.
- `inconclusive` release evidence is partial credit, not pass.
- Missing validation commands produce 0 validation points.
- The report keeps raw summaries visible; maturity does not replace evidence.
- Dashboard rendering remains out of scope.
