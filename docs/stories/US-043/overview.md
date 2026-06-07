# Overview

## Current Behavior

HI-OS has quickstart, clean clone, example workflow, and agent packs, but
common adoption failures are still spread across README, scripts docs, and
story evidence.

## Target Behavior

US-043 adds a troubleshooting guide covering:

- installer failures;
- release verify failures and inconclusive states;
- CodeGraph unavailable;
- NotebookLM auth/session and uncited output;
- governance gate failures;
- governance report/dashboard failures.

## Affected Users

- First-time users debugging adoption.
- Agents recovering from failed governance gates.
- Maintainers diagnosing release/provider evidence.

## Affected Product Docs

- `README.md`
- `docs/README.md`
- `docs/troubleshooting.md`
- `scripts/verify-adoption-docs.py`

## Non-Goals

- Command cookbook.
- Agent pack changes.
- Release/tag/installer pin changes.
- Provider live calls.
- CLI behavior changes.

