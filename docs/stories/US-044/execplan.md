# Exec Plan

## Goal

Add a grouped command cookbook for common HI-OS operations.

## Scope

In scope:

- Create `docs/COMMAND_COOKBOOK.md`.
- Link it from README and docs map.
- Extend adoption verifier.
- Record validation evidence and pass story governance.

Out of scope:

- Troubleshooting guide changes.
- Agent pack changes.
- Release/tag/installer pin changes.
- Provider live calls.
- CLI behavior changes.

## Risk Classification

Risk flags:

- Existing behavior: changes public adoption docs.
- Weak proof: command examples can drift from CLI names.

Hard gates:

- None.

Lane: normal.

## Work Phases

1. Open issue, intake, and story row.
2. Draft cookbook.
3. Link from README and docs map.
4. Extend adoption verifier.
5. Run validation.
6. Record trace.
7. Run story governance gate.
8. Commit, push, and close issue.

## Stop Conditions

Pause for human confirmation if:

- Cookbook requires new CLI commands.
- Cookbook would claim provider unavailable is pass.
- Cookbook would change release or installer behavior.

