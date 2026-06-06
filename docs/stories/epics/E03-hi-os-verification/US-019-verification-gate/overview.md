# Overview

## Current Behavior

Harness can generate context and run a story command, but architecture policy
and handoff completeness are not enforceable.

## Target Behavior

Agents can run an architecture check and a final story verification gate that
returns a non-zero exit code when required governance evidence is missing.

## Affected Users

- AI coding agents.
- Human reviewers.
- Harness maintainers.

## Affected Product Docs

- `docs/HARNESS.md`
- `docs/FEATURE_INTAKE.md`
- `docs/ARCHITECTURE.md`
- `docs/CONTEXT_RULES.md`

## Non-Goals

- Full dependency graph semantics.
- Provider integrations.
