# Cursor Agent Pack

Use this pack when Cursor works in a HI-OS repository.

Cursor can see many files quickly, but HI-OS still requires a governed work
loop. Start from the Harness entrypoints instead of relying on open editor tabs
or chat context alone.

## Startup Checklist

Before editing files:

```text
Read AGENTS.md.
Read README.md.
Read docs/HARNESS.md.
Read docs/FEATURE_INTAKE.md.
Read docs/CONTEXT_RULES.md.
Run harness-cli query matrix.
```

If a story exists, generate or read the story context pack:

```bash
scripts/bin/harness-cli context --story US-XXX
```

On Windows:

```powershell
.\scripts\bin\harness-cli.exe context --story US-XXX
```

## Work Rules

- Do not code before the task has intake, lane, affected docs, and validation
  expectations.
- Keep Cursor edits scoped to the active story.
- Use search and context packs to avoid broad unrelated refactors.
- For normal or high-risk work, keep `docs/stories/US-XXX/` current.
- Treat CodeGraph and NotebookLM as evidence producers, not direct writers to
  Harness SQLite.
- Missing provider, network, auth, notebook, or source context is
  `inconclusive`, never `pass`.
- Do not store Google credentials, cookies, tokens, browser profiles, or
  provider session files in Harness.

## Verification Discipline

Before handoff:

```text
Run the story validation command.
Run harness-cli arch-check --story US-XXX.
Record proof flags with numeric 1/0 values.
Record a trace.
Run harness-cli story verify US-XXX.
```

Cursor should not mark work complete from visual inspection or edited files
alone. Completion requires validation output and story gate evidence.

## Recovery Pattern

When the gate fails, inspect the missing proof row in `harness-cli query
matrix`, update only the missing evidence, then rerun `story verify`.

