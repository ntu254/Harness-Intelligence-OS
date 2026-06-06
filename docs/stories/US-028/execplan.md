# Exec Plan

## Scope

In scope:

- Update v0.4 release notes.
- Confirm CLI help and smoke commands for context ingest, CodeGraph,
  NotebookLM, and auto intake.
- Run full validation ladder.
- Build release binaries and SHA256 assets.
- Publish GitHub release `harness-cli-v0.4.0`.
- Run release verify for `0.4.0` linked to US-028.
- Record CodeGraph evidence and NotebookLM pass-or-inconclusive evidence.
- Generate context pack.
- Record Detailed trace and run story gate.

Out of scope:

- v0.5 friction taxonomy and learning-loop commands.
- v0.6 reporting/dashboard commands.
- Provider credential storage.

## Work Phases

1. Audit version, installer pin, release notes, and workflow state.
2. Decide whether local NotebookLM provider proof is available; if not, record
   documented inconclusive evidence without satisfying NotebookLM pass proof.
3. Run full Rust and schema validation.
4. Build and checksum all release assets.
5. Publish release.
6. Verify release using `harness-cli release verify`.
7. Update story evidence, context pack, trace, and governance gate.
