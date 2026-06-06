# Exec Plan

## Scope

In scope:

- Add a repository/service query for latest passing context ingest mapped
  evidence by story and source.
- Update `intake --auto` to prefer passing ingested evidence.
- Preserve explicit file/text fallback behavior.
- Improve context pack evidence rendering for non-passing ingest diagnostics.
- Add tests for pass, fail, inconclusive, and fallback behavior.

Out of scope:

- Provider invocation.
- New schema migration.
- v0.4 release/tag.
- Installer changes.

## Work Phases

1. Add typed auto-intake evidence structures.
2. Read latest passing ingest reports from SQLite report paths.
3. Merge mapped CodeGraph and NotebookLM context into auto intake defaults.
4. Render ingest failure details in context pack evidence.
5. Add tests proving pass evidence is consumed and non-pass evidence is ignored.
6. Run validation and story governance checks.
