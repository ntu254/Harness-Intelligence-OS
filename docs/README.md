# Documentation Map

This directory holds HI-OS operating docs, adoption guides, governance records,
and any product contract derived from a future user-provided spec.

## Main Files

- `HARNESS.md`: how humans and agents collaborate.
- `FEATURE_INTAKE.md`: how prompts become tiny, normal, or high-risk work.
- `ARCHITECTURE.md`: architecture discovery and boundary rules.
- `TEST_MATRIX.md`: legacy proof map; current proof status is queried with
  `scripts/bin/harness-cli query matrix`.
- `HARNESS_BACKLOG.md`: legacy improvement list; current improvement records
  are stored with `scripts/bin/harness-cli backlog`.
- `GLOSSARY.md`: shared terms.
- `COMMAND_COOKBOOK.md`: grouped examples for intake, context, verification,
  trace, release, dashboard, and MCP commands.
- `troubleshooting.md`: recovery steps for installer, release verification,
  provider, governance gate, and dashboard failures.
- Root `hios.toml`: tracked HI-OS product identity and default public origin.
- Root `packaging/production-include.toml`: production-clean ZIP contract.

## Folders

- `product/`: current product truth, empty until a spec is derived.
- `stories/`: feature packets and backlog.
- `decisions/`: durable decisions and tradeoffs.
- `agents/`: tool-specific operating packs for Codex, Claude Code, and Cursor.
- `demo/`: concrete walkthroughs that show how the harness transforms input
  into agent-ready work.
- `adoption/`: first-run and user-adoption walkthroughs, starting with the
  clean clone path.
- `examples/`: end-to-end workflows that show intake, context, validation,
  trace, story gate, and dashboard evidence together.
- `archive/`: historical phase plans and early specs kept for provenance, not
  current operating policy.
- `templates/`: reusable spec-intake, story, plan, decision, and validation
  formats.

## Current State

HI-OS is the active repository operating layer. Current behavior is defined by
the live docs, decisions, schemas, story packets, CLI code, and validation
evidence; archived plans are historical context only.
