# Governance Report Contract

HI-OS v0.6 uses governance reports as static JSON snapshots of Harness proof
state. The report is evidence for humans and dashboards; it does not mutate
stories, backlog, policy, or release records.

The canonical schema is:

- `docs/schemas/governance-report.schema.json`

## Generation

US-035 adds:

```text
harness-cli governance report --output .harness/reports/governance-report.json
```

If `--output` is omitted, Harness writes:

```text
.harness/reports/governance-report.json
```

Generated reports are validated by:

```text
python scripts/verify-governance-report-schema.py .harness/reports/governance-report.json
```

## Dashboard Export

US-037 adds:

```text
harness-cli governance dashboard \
  --report .harness/reports/governance-report.json \
  --output .harness/dashboard/index.html
```

If `--report` is omitted, Harness reads:

```text
.harness/reports/governance-report.json
```

If `--output` is omitted, Harness writes:

```text
.harness/dashboard/index.html
```

The dashboard is standalone static HTML. It uses no external assets, no live
server, and no script execution. It reads a generated governance report and
writes only the requested HTML file.

## Report Shape

A governance report captures:

- tracked HI-OS product identity from `hios.toml`;
- repository identity and commit metadata;
- story proof summary;
- governance gate summary;
- validation command summary;
- release verification summary;
- friction and suggestion summary;
- governance maturity summary;
- story-level proof rows.

## Maturity Summary

US-036 adds `maturity_summary` to generated reports. The score is deterministic
and integer-only:

- story gate pass rate contributes up to 40 points;
- validation command pass rate contributes up to 25 points;
- release verification contributes up to 20 points;
- low high-severity friction contributes up to 15 points.

Levels are:

- `trusted`: 85-100
- `managed`: 70-84
- `developing`: 50-69
- `early`: 0-49

The summary also records open governance gaps and explanatory notes. It does
not change story status, gates, release proof, friction events, backlog, or
policy.

## Guardrails

- Reports are read-only snapshots.
- Reports include identity evidence, but identity still lives in tracked
  `hios.toml`.
- Missing or failed evidence remains visible; it is not downgraded to a warning.
- `inconclusive` remains distinct from `pass`.
- Runtime report files may live under `.harness/reports/`.
- Runtime dashboard files may live under `.harness/dashboard/`.
- US-034 defines the schema only.
- US-035 implements report generation only.
- US-036 adds maturity scoring.
- US-037 exports static dashboard files only.
