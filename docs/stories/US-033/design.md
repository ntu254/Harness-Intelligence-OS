# Design

## Release Scope

US-033 releases the v0.5 learning-loop foundation:

- US-029 friction taxonomy and schema
- US-030 structured friction capture
- US-031 backlog suggestions from friction
- US-032 rule improvement proposals from friction

## Installer Payload Review

Installer payload must include schema and decision contracts needed by installed
Harness projects:

- MCP artifact schemas from v0.4
- friction event schema from v0.5
- Decision 0010 for MCP file-based artifact boundaries
- Decision 0011 for friction taxonomy
- migration 007 for structured friction events

## Trusted Distribution Flow

```text
version bump
  -> release notes
  -> local validation
  -> tag harness-cli-v0.5.0
  -> GitHub release workflow
  -> 5 platform binaries
  -> 5 SHA256 files
  -> harness-cli release verify --version 0.5.0
  -> story gate
```

## Guardrails

- Existing public v0.4.0 release remains immutable.
- Release verification must check public assets, SHA256, binary version, and
  smoke execution.
- NotebookLM live auth remains external and can stay inconclusive.
- Release hardening must not weaken governance gates.
