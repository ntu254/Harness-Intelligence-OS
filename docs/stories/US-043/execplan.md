# Exec Plan

## Goal

Add a first troubleshooting guide for HI-OS adoption and governance failures.

## Scope

In scope:

- Create `docs/troubleshooting.md`.
- Link it from README and docs map.
- Extend adoption verifier.
- Record validation evidence and pass story governance.

Out of scope:

- Command cookbook.
- Agent pack changes.
- Release/tag/installer pin changes.
- Provider live calls.
- CLI behavior changes.

## Risk Classification

Risk flags:

- Existing behavior: changes public adoption docs.
- Weak proof: troubleshooting must not weaken failure semantics.

Hard gates:

- None.

Lane: normal.

## Work Phases

1. Open issue, intake, and story row.
2. Draft troubleshooting guide.
3. Link guide from entrypoints.
4. Extend adoption verifier.
5. Run validation.
6. Record trace.
7. Run story governance gate.
8. Commit, push, and close issue.

## Stop Conditions

Pause for human confirmation if:

- A recovery step would bypass checksum verification.
- A recovery step would convert `fail` or `inconclusive` into `pass`.
- A recovery step would store provider credentials in Harness.

