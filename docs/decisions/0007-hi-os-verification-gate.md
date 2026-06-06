# 0007 HI-OS Verification Gate

Date: 2026-06-06

## Status

Accepted

## Context

HI-OS v0.1 creates grounded context before implementation, but Harness still
cannot make a mechanical pass/fail decision at handoff. Story verification only
runs a configured shell command, and architecture rules remain prose.

## Decision

HI-OS v0.2 adds two repository-local controls:

1. `arch-check` scans configured layer paths and forbidden import segments.
2. `story verify` preserves mechanical command execution and then evaluates a
   governance checklist covering intake, context, architecture, validation
   evidence, automated impact evidence, and trace presence.

Architecture results are stored on the story. High-risk stories require
validation evidence plus at least one proof flag. The final governance gate runs
after the task trace is recorded.

## Alternatives Considered

1. Replace mechanical story verification with checklist-only verification.
   Rejected because existing verify commands remain valuable executable proof.
2. Require a full CodeGraph dependency adapter immediately. Rejected because a
   deterministic import scanner is sufficient for the first enforceable gate.
3. Keep the gate advisory. Rejected because HI-OS v0.2 is intended to authorize
   or reject handoff.

## Consequences

- Story handoff gains a durable pass/fail result.
- Architecture policy becomes executable and project-configurable.
- Existing workflows must record a trace before the final story verification.
- The scanner is intentionally narrower than a semantic dependency graph.
