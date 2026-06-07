# Overview

## Current Behavior

The repository has strong governance, release verification, MCP evidence, and
dashboard commands, but a new user starting from a clean clone must infer the
first working path from scattered docs.

A clean clone also starts without `harness.db`, which can make `query matrix`
look empty until the user understands that durable state is local and ignored.

## Target Behavior

US-039 adds a clean clone walkthrough that shows the first complete local loop:

- clone the repository;
- build the Harness CLI;
- initialize and import local durable state;
- create a local demo intake and story;
- generate context;
- run architecture and story governance gates;
- export governance report and dashboard;
- optionally verify the public release;
- understand provider-unavailable behavior for CodeGraph and NotebookLM.

## Affected Users

- First-time maintainers evaluating HI-OS.
- Agents entering the repo from a clean checkout.
- Contributors who need to distinguish tracked source changes from ignored
  runtime evidence.

## Affected Product Docs

- `README.md`
- `docs/README.md`
- `docs/adoption/clean-clone-walkthrough.md`
- `scripts/README.md`

## Non-Goals

- Full README quickstart rewrite.
- Command cookbook.
- Troubleshooting guide.
- Cursor, Claude, or Codex instruction packs.
- v0.7 release/tag/installer pin changes.

