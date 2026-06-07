# Overview

## Current Behavior

README explains the older `repository-harness` framing first and contains many
installer details before a new reader understands the current HI-OS workflow.

US-039 added a clean clone walkthrough, but the README still does not function
as a five-minute product landing page for HI-OS adoption.

## Target Behavior

README presents HI-OS as the primary identity and lets a new reader understand
the project in about five minutes:

- what HI-OS is;
- why agent-ready repositories need governance;
- how to install it;
- how the quickstart moves through intake, context, verify, trace, and
  dashboard;
- how release verification works;
- what the governance dashboard shows;
- how CodeGraph and NotebookLM fit into the file-based evidence boundary;
- where to go next for clean clone setup.

## Affected Users

- First-time users evaluating HI-OS.
- Maintainers installing HI-OS into a project.
- Agents using README as high-level orientation before deeper Harness docs.

## Affected Product Docs

- `README.md`
- `scripts/verify-adoption-docs.py`
- `docs/stories/US-040/validation.md`

## Non-Goals

- Full workflow example document.
- Agent instruction packs.
- Troubleshooting guide.
- Command cookbook.
- Sovereign identity CLI/config changes.
- v0.7 release/tag/installer pin changes.

