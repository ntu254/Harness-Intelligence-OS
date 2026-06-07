# Exec Plan

## Goal

Create agent instruction packs for Cursor, Claude Code, and Codex.

## Scope

In scope:

- Add `docs/agents/cursor.md`.
- Add `docs/agents/claude-code.md`.
- Add `docs/agents/codex.md`.
- Link packs from `AGENTS.md`, `README.md`, and `docs/README.md`.
- Extend `scripts/verify-adoption-docs.py`.
- Record validation evidence and pass story governance.

Out of scope:

- Installer propagation.
- General troubleshooting guide.
- Command cookbook.
- Release/tag/installer pin changes.
- CLI behavior changes.

## Risk Classification

Risk flags:

- Existing behavior: changes agent-facing instructions.
- Weak proof: docs can drift without verifier coverage.

Hard gates:

- None.

Lane: normal.

## Work Phases

1. Open issue, intake, and story row.
2. Draft three agent packs.
3. Link packs from entrypoint docs.
4. Extend adoption verifier.
5. Run validation.
6. Record trace.
7. Run story governance gate.
8. Commit, push, and close issue.

## Stop Conditions

Pause for human confirmation if:

- Installer behavior must change.
- The packs would weaken validation or source-of-truth rules.
- A pack would require storing credentials or provider sessions.

