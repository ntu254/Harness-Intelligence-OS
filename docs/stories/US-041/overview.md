# Overview

## Current Behavior

US-039 gives a clean clone walkthrough and US-040 explains HI-OS from README,
but users still do not have one compact example showing a complete agent story
from intake through dashboard.

## Target Behavior

US-041 adds `docs/examples/full-agent-workflow.md`, an end-to-end example that
shows:

- agent context reading before edits;
- intake recording;
- story creation;
- optional CodeGraph and NotebookLM provider context;
- context pack generation;
- minimal implementation;
- validation proof;
- proof flag update;
- trace recording;
- story governance gate;
- governance report and dashboard export;
- provider troubleshooting when CodeGraph or NotebookLM is unavailable.

## Affected Users

- First-time users learning the full HI-OS loop.
- Agents that need a concrete handoff pattern.
- Maintainers explaining governance evidence to contributors.

## Affected Product Docs

- `README.md`
- `docs/README.md`
- `docs/examples/full-agent-workflow.md`
- `scripts/verify-adoption-docs.py`

## Non-Goals

- Agent instruction packs.
- General troubleshooting guide.
- Command cookbook.
- Release/tag/installer pin changes.
- Live CodeGraph or NotebookLM provider calls.
- CLI behavior changes.

