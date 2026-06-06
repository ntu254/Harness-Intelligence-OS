# Exec Plan

## Goal

Make HI-OS capable of passing or failing story handoff.

## Scope

In scope:

- Configurable architecture import scanning.
- Durable architecture results linked to stories.
- Governance checks in `story verify`.
- Installer, schema, tests, and operating docs.

Out of scope:

- A real CodeGraph adapter.
- NotebookLM provenance enforcement.
- Semantic dependency analysis.

## Risk Classification

Risk flags:

- Architecture.
- Existing behavior.
- Weak proof.

Hard gates:

- Validation requirements change.

## Work Phases

1. Inspect the current CLI and durable schema.
2. Define the compatibility-preserving command contract.
3. Add migrations and implementation.
4. Add focused scanner and gate tests.
5. Update installers and policy docs.
6. Run release checks and record trace evidence.

## Stop Conditions

Pause if the implementation would remove mechanical verification or weaken
existing proof requirements.
