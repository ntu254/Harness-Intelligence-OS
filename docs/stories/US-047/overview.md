# US-047: Prune Legacy Repository-Harness Artifacts

## Status

Implemented / High-Risk

## Issue

- GitHub issue: #24
- Milestone: HI-OS v0.7.0 Adoption Ready

## Goal

Remove or archive legacy repository-harness artifacts so HI-OS feels like a
sovereign project without losing durable governance evidence.

## Problem

US-046 established tracked HI-OS identity, but the repository still contains
older repository-harness framing and historical artifacts. Some of those are
valuable evidence and should stay. Some should be migrated to HI-OS language.
Some should be archived. Some generated or mock artifacts should not remain in
tracked source.

US-047 makes that separation explicit.

## In Scope

- Audit the repo with `rg`.
- Create a KEEP / MIGRATE / ARCHIVE / DELETE cleanup ledger.
- Preserve decisions, stories, schemas, trace evidence, and release evidence.
- Migrate primary docs away from legacy repository-harness framing.
- Archive historical documents that still explain project evolution.
- Remove obsolete mock/runtime/generated files if any are tracked.
- Run full validation and story gate.

## Out Of Scope

- v0.7 release hardening.
- Release tag, installer pin, or public asset changes.
- MCP/provider behavior changes.
- Credential/session storage.
