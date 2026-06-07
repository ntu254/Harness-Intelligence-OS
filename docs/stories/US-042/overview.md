# Overview

## Current Behavior

HI-OS has general agent instructions in `AGENTS.md`, but there are no
agent-specific packs for Cursor, Claude Code, or Codex. New users must infer
how each agent should apply the same Harness workflow.

## Target Behavior

US-042 adds agent instruction packs:

- `docs/agents/cursor.md`;
- `docs/agents/claude-code.md`;
- `docs/agents/codex.md`.

Each pack explains required context reading, context pack use, lane/story
discipline, verification expectations, provider boundary rules, and handoff
behavior.

## Affected Users

- Maintainers configuring HI-OS for a specific coding agent.
- Agents entering the repository from tool-specific context surfaces.
- Reviewers auditing whether agent handoff followed Harness proof rules.

## Affected Product Docs

- `AGENTS.md`
- `README.md`
- `docs/README.md`
- `docs/agents/*`
- `scripts/verify-adoption-docs.py`

## Non-Goals

- Changing installer-generated agent files.
- Changing `CLAUDE.md` import behavior.
- Troubleshooting guide.
- Command cookbook.
- Release/tag/installer pin changes.
- CLI behavior changes.

