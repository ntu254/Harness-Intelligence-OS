# Overview

## Current Behavior

Common commands are spread across README, scripts docs, examples, and story
evidence. Users have to search when they need a quick command reference.

## Target Behavior

US-044 adds `docs/COMMAND_COOKBOOK.md`, grouped by:

- setup and state;
- intake;
- stories;
- context;
- verify;
- trace;
- release;
- dashboard;
- MCP/provider evidence;
- friction and learning loop.

## Affected Users

- First-time users moving from quickstart to daily use.
- Agents needing accurate command shapes.
- Maintainers verifying common governance evidence.

## Affected Product Docs

- `README.md`
- `docs/README.md`
- `docs/COMMAND_COOKBOOK.md`
- `scripts/verify-adoption-docs.py`

## Non-Goals

- Troubleshooting guide changes.
- Agent pack changes.
- Release/tag/installer pin changes.
- Provider live calls.
- CLI behavior changes.

